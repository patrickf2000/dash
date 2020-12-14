
#OUTPUT
#X: ffffffa4
#Y: 55
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func print_two(x:byte, y:byte)
    printf("X: %x\n", x)
    printf("Y: %x\n", y)
end

func main -> int
    byte x = 0xA4
    print_two(x, 0x55)
    
    return 0
end
