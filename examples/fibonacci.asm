; Set initial values to 1
LD 1C ; acc = 1
ST 0A ; a = acc
ST 1A ; b = acc

; loop_start
LD 0A ; acc = a
ADD 1A ; acc = acc + b
ST 2A ; c = acc

LD 1A ; acc = b
ST 0A ; a = acc

LD 2A ; acc = c
ST 1A ; b = acc

BRA 3A ; goto loop_start