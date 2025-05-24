# ((P-Q*R)/S) +((T/U)+V*W)
# Program using 3 adressing mode

MUL R1,Q,R
SUB R1,P,R1
DIV R1,R1,S
DIV R2,T,U
MUL R3,V,W
ADD R2,R3,R2
ADD N,R2,R1




