package com.algorithmia;

import com.google.gson.*;
import java.lang.reflect.*;
import java.lang.annotation.*;
import java.io.*;
import java.io.FileOutputStream;
import java.io.PrintStream;
import java.util.Collection;
import java.util.Scanner;
import java.util.*;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Files;
import java.nio.file.Paths;
import org.apache.commons.io.FileUtils;
import org.apache.commons.io.IOUtils;
import java.util.function.Function;
public class SignatureUtilities {
    public static final String METHOD_KEY_BYTE_ARRAY = "Bytes-";
    public static final String METHOD_KEY_OBJECT = "Object-";

    static class FloatSerializer implements JsonSerializer<Float> {
        public JsonElement serialize(Float value, Type theType, JsonSerializationContext context) {
            if (value.isInfinite() || value.isNaN()) {
                return new JsonPrimitive(value.toString());
            } else {
                return new JsonPrimitive(value);
            }
        }
    }
    static class DoubleSerializer implements JsonSerializer<Double> {
        public JsonElement serialize(Double value, Type theType, JsonSerializationContext context) {
            if (value.isInfinite() || value.isNaN()) {
                return new JsonPrimitive(value.toString());
            } else {
                return new JsonPrimitive(value);
            }
        }
    }

    public static Gson gson = new GsonBuilder()
        .serializeNulls()
        .registerTypeAdapter(Float.class, new FloatSerializer())
        .registerTypeAdapter(Double.class, new DoubleSerializer())
        .create();


    private static Character stringToCharacter(String s) {
        if (s.length() == 1) {
            return s.charAt(0);
        }
        throw new IllegalStateException("cannot convert to char");
    }

    private static Function<JsonElement, Double> toDouble = number -> number.getAsNumber().doubleValue();
    private static Function<JsonElement, Float> toFloat = number -> number.getAsNumber().floatValue();
    private static Function<JsonElement, Long> toLong = number -> number.getAsNumber().longValue();
    private static Function<JsonElement, Integer> toInteger = number -> number.getAsNumber().intValue();
    private static Function<JsonElement, Short> toShort = number -> number.getAsNumber().shortValue();
    private static Function<JsonElement, Character> toCharacter = str -> stringToCharacter(str.getAsString());
    private static Function<JsonElement, Byte> toByte = number -> number.getAsNumber().byteValue();
    private static Function<JsonElement, Boolean> toBoolean = bool -> bool.getAsBoolean();
    private static Function<JsonElement, byte[]> toByteArray = jsonElement -> Base64.getDecoder().decode(jsonElement.getAsString());
    private static Function<JsonElement, String> toString = obj -> obj.getAsString();

    private static Function<JsonElement, Object> getCoerceFunction(Type t) {
        Function<JsonElement, Object> result = input -> gson.fromJson(input, t);
        return result;
    }

    public static int getPriority(Function f) {
        if (f == toDouble) {
            return 1;
        } else if (f == toFloat) {
            return 2;
        } else if (f == toLong) {
            return 3;
        } else if (f == toInteger) {
            return 4;
        } else if (f == toShort) {
            return 5;
        } else if (f == toString) {
            return 6;
        } else if (f == toCharacter) {
            return 7;
        } else if (f == toByte) {
            return 8;
        } else if (f == toBoolean) {
            return 9;
        } else if (f == toByteArray) {
            return 10;
        }

        return 11;  // can we order these coerced functions?
    }

    public static MethodData getMethodData(Method m) {
        StringBuffer signature = new StringBuffer();
        Class<?>[] parameterTypes = m.getParameterTypes();
        Type[] genericParameterTypes = m.getGenericParameterTypes();
        Function[] conversions = new Function[parameterTypes.length];

        for (int i = 0; i < parameterTypes.length; i++) {
            Class<?> c = parameterTypes[i];
            if (c.isPrimitive()) {
                if (c == byte.class){
                    signature.append("Number-");
                    conversions[i] = toByte;
                } else if (c == short.class) {
                    signature.append("Number-");
                    conversions[i] = toShort;
                } else if (c == int.class) {
                    signature.append("Number-");
                    conversions[i] = toInteger;
                } else if (c == long.class) {
                    signature.append("Number-");
                    conversions[i] = toLong;
                } else if (c == float.class) {
                    signature.append("Number-");
                    conversions[i] = toFloat;
                } else if (c == double.class) {
                    signature.append("Number-");
                    conversions[i] = toDouble;
                } else if (c == char.class) {
                    signature.append("String-");
                    conversions[i] = toCharacter;
                } else if (c == boolean.class) {
                    signature.append("Boolean-");
                    conversions[i] = toBoolean;
                } else {
                    signature.append(METHOD_KEY_OBJECT);
                    conversions[i] = getCoerceFunction(genericParameterTypes[i]);
                }
            } else {
                conversions[i] = getCoerceFunction(genericParameterTypes[i]);
                if (c.getSuperclass() == Number.class) {
                    if (c == Byte.class){
                        conversions[i] = toByte;
                    } else if (c == Short.class) {
                        conversions[i] = toShort;
                    } else if (c == Integer.class) {
                        conversions[i] = toInteger;
                    } else if (c == Long.class) {
                        conversions[i] = toLong;
                    } else if (c == Float.class) {
                        conversions[i] = toFloat;
                    } else if (c == Double.class) {
                        conversions[i] = toDouble;
                    } else {
                        conversions[i] = getCoerceFunction(genericParameterTypes[i]);
                    }
                    signature.append("Number-");

                    // Make a function to coerce?
                } else if (c == Boolean.class) {
                    signature.append("Boolean-");
                    conversions[i] = toBoolean;
                } else if (c == byte[].class) {
                    signature.append(METHOD_KEY_BYTE_ARRAY);
                    conversions[i] = toByteArray;
                } else if (c == Character.class) {
                    signature.append("String-");
                    conversions[i] = toCharacter;
                } else {
                    try {
                        Object pInstance = c.newInstance();
                        if (pInstance instanceof Number) {
                            signature.append("Number-");
                        } else if (pInstance instanceof String) {
                            signature.append("String-");
                            conversions[i] = toString;
                        } else {
                            signature.append(METHOD_KEY_OBJECT);
                        }
                    } catch (Exception e) {
                        // Skip making this guy and say it's an object
                        signature.append(METHOD_KEY_OBJECT);
                    }
                }
            }
        }

        String key = signature.toString();
        return new MethodData(m, key, conversions);
    }

    public static String getGenericKey(int argsSize) {
        return "" + argsSize;
    }

    public static MethodData getGenericMethodData(Method m) {
        StringBuffer signature = new StringBuffer();
        Type[] genericParameterTypes = m.getGenericParameterTypes();
        Function[] conversions = new Function[genericParameterTypes.length];

        for (int i = 0; i < genericParameterTypes.length; i++) {
            conversions[i] = getCoerceFunction(genericParameterTypes[i]);
        }

        String key = getGenericKey(conversions.length);
        return new MethodData(m, key, conversions);
    }

    public static String getMethodKey(JsonArray array) {
        StringBuffer inputTypes = new StringBuffer();

        for (int i = 0; i < array.size(); i++) {
            JsonElement cur = array.get(i);
            inputTypes.append(getMethodKeyForElement(cur));
        }

        return inputTypes.toString();
    }

    public static String getMethodKeyForElement(JsonElement cur) {
        if (cur.isJsonPrimitive()) {
            JsonPrimitive primative = cur.getAsJsonPrimitive();
            if (primative.isBoolean()) {
                return "Boolean-";
            } else if (primative.isNumber()) {
                return "Number-";
            } else if (primative.isString()) {
                // Since we can't tell the difference between strings and characters here, we lump them together
                return "String-";
            } else {
                return METHOD_KEY_OBJECT;
            }
        } else {
            return METHOD_KEY_OBJECT;
        }
    }
}