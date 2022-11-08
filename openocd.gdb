target remote :3333
monitor arm semihosting enable

break DefaultHandler
break HardFault

load
