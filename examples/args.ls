
# This an example of what parsing command line arguments in Lila could look like
#
# As you can see, we don't need argc and argv like in C. All we need is a string
# list. The size can be retrieved with the sizeof() operator

use std.io;

func main(args:str[]) -> int
    i, size : int = 0;
    current : str = "";
begin
    size = sizeof(args);
    printf("Argc: %d\n\n", size);
    
    while i < size
        current = args[i];
        println(current);
        
        i = i + 1;
    end
    
    return 0;
end
