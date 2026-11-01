def encontrar_suma(datos, objetivo):
    tabla_hash = {}
    
    for i in range(len(datos)):
        complemento = objetivo - datos[i]
        
        if complemento in tabla_hash:
            return [tabla_hash[complemento], i]
            
        tabla_hash[datos[i]] = i
        
    return []

arreglo = [2, 14, 6, 11, 8, 15, 7]
target = 9

resultado = encontrar_suma(arreglo, target)
print("Índices encontrados:", resultado)