def busqueda_binaria(datos, buscado):
    lim_inf = 0
    lim_sup = len(datos) - 1
    no_encontrado = True

    while lim_inf <= lim_sup:
        mid = (lim_inf + lim_sup) // 2

        if datos[mid] == buscado:
            print(f"el dato {buscado} se encontró en la posicion {mid}")
            no_encontrado = False
            break
        elif buscado < datos[mid]:
            lim_sup = mid - 1
        else:
            lim_inf = mid + 1

    if no_encontrado:
        print("No existe en la lista")


valores_n = [10, 100, 1000, 10000, 100000, 1000000]

for n in valores_n:
    info = [x * 5 for x in range(1, n + 1)]
    busqueda_binaria(info, n * 5)
