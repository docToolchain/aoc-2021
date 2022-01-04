import numpy as np

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    bin_digits = [[int(j) for j in list(code)] for code in lines_stripped] 
    
    return bin_digits

def main():
    with open("input.txt",'r') as diagnostic_file:
        diagnostic_lines = diagnostic_file.readlines()

    diagnostics = process_input(diagnostic_lines)
    
    diag_matrix = np.matrix(diagnostics)
    matrix_size = diag_matrix.shape
    no_codes = matrix_size[0]
    sum_diag = diag_matrix.sum(0)
    sum_diag = sum_diag.tolist()[0]

    gamma = int(''.join([str(int(j // (no_codes/2))) for j in sum_diag]),2)
    epsilon = int(''.join([str(int(j // (no_codes/2)+1)%2) for j in sum_diag]),2)
        
    print(gamma, epsilon, gamma*epsilon)
    
    if gamma<2**(matrix_size[1]-1):
        majority_oxy = 0
        majority_co2 = 1
    else:
        majority_oxy = 1
        majority_co2 = 0

    candidates_oxy = diagnostics.copy()
    candidates_co2 = diagnostics.copy()
    i = 0
    
    while len(candidates_oxy)!=1 or len(candidates_co2)!=1:
        oxy_temp = candidates_oxy.copy()
        co2_temp = candidates_co2.copy()
        if len(candidates_oxy) != 1:
            for code in oxy_temp:
                if code[i]!=majority_oxy:
                    candidates_oxy.remove(code)
        if len(candidates_co2) != 1:
            for code in co2_temp:
                if code[i]!=majority_co2:
                    candidates_co2.remove(code)
        if i != matrix_size[1]-1:
            i += 1
            oxy_matrix = np.matrix(candidates_oxy)
            sum_oxy = oxy_matrix.sum(0)
            sum_oxy = sum_oxy.tolist()[0]
            
            co2_matrix = np.matrix(candidates_co2)
            sum_co2 = co2_matrix.sum(0)
            sum_co2 = sum_co2.tolist()[0]
            
            if sum_oxy[i] >= len(candidates_oxy)/2:
                majority_oxy = 1
            else:
                majority_oxy = 0
            if sum_co2[i] < len(candidates_co2)/2:
                majority_co2 = 1
            else:
                majority_co2 = 0

    c_oxy = int(''.join([str(i) for i in candidates_oxy[0]]),2)
    c_co2 = int(''.join([str(i) for i in candidates_co2[0]]),2)
    
    print(c_oxy, c_co2, c_oxy*c_co2)

main()
