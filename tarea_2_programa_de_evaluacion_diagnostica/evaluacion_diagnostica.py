# declaracion de estructura
class Nodo:
    def __init__(self, valor):
        self.valor = valor
        self.izq = None
        self.centro = None
        self.der = None

#representacion de datos
n20 = Nodo(20)
n23 = Nodo(23)
n19 = Nodo(19)
n57 = Nodo(57)
n67 = Nodo(67)
n99 = Nodo(99)

head = n20

n20.izq = n23
n20.centro = n19

n23.centro = n57

n19.der = n67

n67.der = n99

#imprimo nodo 99 y 57
print(head.centro.der.der.valor)
print(head.izq.centro.valor)
