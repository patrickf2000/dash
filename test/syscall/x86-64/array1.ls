
#OUTPUT
#Correct!
#END

#RET 0

func free -> int
    return 0
end

func _start
    int[10] numbers = array
    
    numbers[3] = 22
    
    int x = numbers[3]
    
    if x == 22
        syscall(1, 1, "Correct!\n", 9)
    end
    
    exit
end

