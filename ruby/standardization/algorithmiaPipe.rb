

class AlgorithmPipe
    attr_accessor :apply_method
    attr_accessor :on_load_method
    public 
    def initialize(apply_method=nil)
        @apply_method = apply_method
        @on_load_method = nil
    end
    def set_apply_method(apply_method)
        @apply_method = apply_method
    end
    def set_on_load_method(on_load_method)
        @on_load_method = on_load_method
    end
    def run()
        puts "Not Implemented."
    end
end