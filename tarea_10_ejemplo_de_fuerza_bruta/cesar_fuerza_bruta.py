alfabeto = "abcdefghijklmnñopqrstuvwxyz"
n = len(alfabeto)

with open("mensajeCifrado.txt", "r", encoding="utf-8") as f:
    cifrado = f.read()

for k in range(1, n + 1):
    salida = []
    for c in cifrado:
        es_mayus = c.isupper()
        caracter = c.lower()
        if caracter in alfabeto:
            idx = (alfabeto.index(caracter) - k) % n
            nueva_letra = alfabeto[idx]
            salida.append(nueva_letra.upper() if es_mayus else nueva_letra)
        else:
            salida.append(c)

    print(f"[+] Desplazamiento k = {k}:")
    print("".join(salida))
    print("-" * 50)