import Algorithmia
import spacy

"""
This package set comes preinstalled with every available small language package provided by spacy.
Pick your language from the following list: 'en_core_web_sm', 'es_core_news_sm', 'pt_core_news_sm', 'fr_core_news_sm', 'it_core_news_sm', 'de_core_news_sm', and 'nl_core_news_sm'
You may change the language at runtime, but bear in mind that you'll get hit with some performance loss.

Example Input:
"I like New York in Autumn, the trees are beautiful."

Expected Output:
{
    "entities found": [
        {"label": "GPE", "text": "New York"},
        {"label": "DATE", "text": "Autumn"}
    ]
}
"""

LANG = "en_core_web_sm"

def load_spacy_lang(language):
    lang_model = spacy.load(language)
    return lang_model


def apply(input):
    """
    This algorithm performs "Named Entity Recognition" on the sample input document.
    It expects the input to be an escaped string.
    :param input: An escaped string, in the language defined by $LANG.
    :return: a list of detected entities.
    """
    document = nlp(input)
    named_entities = []
    for ent in document.ents:
        entity = {"label": ent.label_, "text": ent.text}
        named_entities.append(entity)
    output = {"entities found": named_entities}
    return output


nlp = load_spacy_lang(LANG)
