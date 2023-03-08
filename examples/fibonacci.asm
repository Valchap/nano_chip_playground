; Constants
$a 0 ; Memory address 0
$b 1 ; Memory address 1
$c 2 ; Memory address 2

; Set initial values to 1
LD 1 ; acc = 1
ST [$a] ; a = acc
ST [$b] ; b = acc

:loop_start
LD [$a]  ; acc = a
ADD [$b] ; acc = acc + b
ST [$c]  ; c = acc

LD [$b] ; acc = b
ST [$a] ; a = acc

LD [$c] ; acc = c
ST [$b] ; b = acc

BRA :loop_start ; goto loop_start