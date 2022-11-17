/*

                    |
                    |
                    |
                    |                          . 12 Puntos de control = Polinomio en piezas de grado 12.
                    |                         /  Como P(1,3) = P(2,0) y P(2,3) = P(3,0), el grado real es solo 10.
                    |           i = 1        /       
                    |                       /
                    |              _..o P  /
                    |            _/     \ /
                    |          ./         .                                 
                    |         /            o. P(1,3) = P(2,0)    ___     i = 3
                    |        o P(1,1)      l \.                +    l+      
                    |       /              l   \             /      l  \
                    |      |               l    \  i = 2   +        l    .
                    |     |                l     \      ./          l      \
                    |    |                 l       . _--            l        +              m = 3 segmentos 
                    |   |                  l                        l         \             "i" va de 1 a m - 1 <-- recorre los segmentos C(u)
                    |   o P(1,0)           l                        l          \            u E [0,1]
    breakpoints ------> u0 = 0            u1                       u2          u3 = 1       u0 < u1 < u2 < u3
                    |_________________________________________________________________________________

                        P(j,i) -> j = segmento
                               -> i = punto (del segmento)
                        
                        u1 y u2 pueden ser discontinuos.

    U = { a, ..., a, u(p+1), ..., u(m-p-1), b, ..., b}
         └────┬────┘                       └────┬────┘
           (p + 1)                           (p + 1)


    p = grado                           ┐
    (n + 1) = no. puntos de control     ├──  m = n + 1 + p  ---> segmentos = no.ptos control + grado
    (m + 1) = no. nodos                 ┘    
                                             m + 1 = (n + 1) + p + 1
                                             no.nodos = no. ptos de control + grado + 1


        Ejemplo: pag. 81
    U = {0,0,0,1,2,3,4,4,5,5,5}         0    1    2    3    4    5    6    7    8    9    10
    let knot_vec:Vec<f64> = Vec::from([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0]);

        Ejemplo: pag. 88
    U = {0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1}
    let knot_vec:Vec<f64> = Vec::from([0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0]);
    let knot_vec:Vec<f64> = Vec::from([0.0,0.0,0.0,0.125,0.25,0.375,0.5,0.625,0.75,0.875,1.0,1.0,1.0]);

    ctrl_points.push(Point::new(0.0, 5.0));
    ctrl_points.push(Point::new(1.0, 7.0));
    ctrl_points.push(Point::new(2.0, 5.0));
    ctrl_points.push(Point::new(4.0, 10.0));
    ctrl_points.push(Point::new(8.0, 9.0));
    ctrl_points.push(Point::new(10.0, 0.0));
    ctrl_points.push(Point::new(14.0, 5.0));
    ctrl_points.push(Point::new(15.0, 3.0));
    ctrl_points.push(Point::new(19.0, 8.0));
    ctrl_points.push(Point::new(20.0, 7.0));
*/