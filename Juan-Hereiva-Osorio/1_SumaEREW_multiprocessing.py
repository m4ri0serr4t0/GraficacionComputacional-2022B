import multiprocessing
import math
import time

def SumaEREW(i,j,A):
    if (((2*j)%(math.pow(2,i))) == 0):
        A[2*j] = A[2*j] + A[((2*j) -((int)(math.pow(2,i-1))))]
        return
    time.sleep(0.5)

if __name__ == '__main__':
    print('''
     SUMA EREW Multiprocessing
             ''')
    print('JUAN HEREIVA OSORIO')
    print("")
    time.sleep(0.5)

    A = [0,5,2,10,1,8,12,7,3]

    print('Vector Ingresado: ')
    print(A, '\n')
    time.sleep(1)

    print('Proceso de suma: \n')

    Process_jobs = []
    n = len(A)-1
    log = int(math.log(n,2))

    for i in range(1,log+1):
        for j in range(1,(int)(n/2)+1):
            process = multiprocessing.Process(target=SumaEREW, args=(i,j,A,))
            Process_jobs.append(process)
            process.run()
            #print('\t', p.is_alive)
            process.start()
            #print('\t', p.is_alive)
            process.join()
            #print('\t', p.is_alive)

        print(A,'\t',process.is_alive)
        print("")
    print('suma total es: ',A[n],'\t',process.is_alive)
    time.sleep(0.5)