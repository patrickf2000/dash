
#OUTPUT
#11223344
#aabbccdd
#11118888
#END

#RET 0

extern func puts(s:str)

func main -> int
    int x = 0x11223344
    
    printf("%x\n", x)
    printf("%x\n", 0xAABBCCDD)
    
    x = 0x11118888
    printf("%x\n", x)
    
    return 0
end