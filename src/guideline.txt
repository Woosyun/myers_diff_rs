import fs from 'fs';

class Diagonals {
    mid: number;
    diagonals: Array<number>;
    
    constructor(xLen: any, yLen: any, fill: any) {
        this.mid = yLen + 1;
        this.diagonals = new Array(2*(xLen + yLen) + 1).fill(fill);
    }
    get(idx: any) {
        return this.diagonals[this.mid + idx];
    }
    set(idx: any, val: any) {
        this.diagonals[this.mid + idx] = val;
    }
}

function loadFile(filePath: any) {
    try {
        const data = fs.readFileSync(filePath, 'utf8');
        return data;        
    } catch (error: any) {
        console.log('Error:', error.stack);
    }
}

/*
Constant MAX = [0, M + N]
Var V: number[] = Array(-MAX, MAX)
V[1] = 0
for D = 0 to MAX
    for k = -D to D in steps of 2
        if k = -D or k != D and V[k-1] < V[k+1] 
            x = V[k+1]
        else
            x = V[k-1] + 1
        y = x - k
        while x < N and y < M and A[x+1] = B[y+1] 
            x, y = x+1, y+1
        V[k] = x
        if x >= N and y >= M
            return D
Length of an SES is greater than MAX
*/
function GreedyLCS(A: any, B: any) {
    const MAX = A.length + B.length;
    let V = new Diagonals(A.length, B.length, 0);
    V.set(1, 0);
    for (let D = 0; D <= MAX; D++) {
        for (let k = -D; k <= D; k += 2) {
            let x, y;
            if (k === -D || k !== D && V.get(k-1) < V.get(k+1))
                x = V.get(k+1);
            else
                x = V.get(k-1) + 1;
            y = x - k;
            while (x < A.length && y < B.length && A[x + 1] === B[y + 1]) {
                x++;
                y++;
            }
            V.set(k, x);
            if (x >= A.length && y >= B.length)
                return D;
        }
    }
    //length of SES is greater than MAX, but that's not gonna happen
    return null;
}

/*
delta = N - M
for D = 0 ~ Ceil((M+N)/2)
    for k = -D ~ D; k += 2
        find the end of the furthest reaching forward D-path in diagonal k
        if delta is odd and delta - (D-1) <= k <= delta + (D-1)
            if the path overlaps the furthest reaching reverse (D-1)-path in diagonal k
                Length of an SES is 2D - 1
                The last snake of the forward path is the middle snake
    for k = -D ~ D; k += 2
        find the end of the furthest reaching reverse D-path in diagonal k + delta
        if delta is even and -D < = k + delta <= D
            if the path overlaps the furthest reaching forward D-path in diagonal k + delta
                Length of an SES is 2D
                The last snake of the reverse path is the middle snake
*/
function FindMiddleSnake(A: any, N: any, B: any, M: any) {
    const delta = N - M;
    const odd = delta % 2 === 1;

    let VForward = new Diagonals(N, M, 0);
    let VReverse = new Diagonals(N, M, N);
    let x: any, y: any, u: any, v: any;
    let result = {
        x: null,
        y: null,
        u: null,
        v: null
    };
    for (let D = 0; D <= Math.ceil((M + N) / 2); D++) {
        for (let k = -D; k <= D; k += 2) {
            //1. find end of furthest reaching forward D-path in diagonal k
            if (k === -D || (k !== D && VForward.get(k + 1) > VForward.get(k - 1)))
                u = VForward.get(k + 1);
            else
                u = VForward.get(k - 1) + 1;
            v = u - k;
            x = u;
            y = v;

            while (u < N && v < M && A[u] === B[v]) {
                u++;
                v++;
            }
            VForward.set(k, u);

            if (x < u && y < v)//if length of snake is greater than 0
                result = { x, y, u, v };
            if (odd && delta - D < k && k < delta + D) {
                if (u >= VReverse.get(k))
                    return result;
            }
            
            // console.log(`x: ${x}, y: ${y}, u: ${u}, v: ${v}`);
            // console.log('snake is ', result);
        }

        for (let k = -D; k <= D; k += 2 ) {    
            //2. find end of furthest reaching reverse D-path in diagonal k+delta
            const tempD = k + delta;
            if (k === D || (k !== -D && VReverse.get(tempD - 1) < VReverse.get(tempD + 1)))
                x = VReverse.get(tempD - 1);
            else
                x = VReverse.get(tempD + 1) - 1;
            y = x - tempD;
            u = x;
            v = y;

            while (x > 0 && y > 0 && A[x - 1] === B[y - 1]) {
                x--;
                y--;
            }
            VReverse.set(tempD, x);
            if (x < u && y < v)
                result = { x, y, u, v };

            if (!odd && -D <= tempD && tempD <= D) {
                if (x <= VForward.get(tempD))
                    return result;
            }

            // console.log(`x: ${x}, y: ${y}, u: ${u}, v: ${v}`);
        }
    }
}

/*
LCS (A, N, B, M) {
    if N > 0 and M > 0 {
        find the middle snake and length of an optimal path for A and B. Suppose it is from (x, y) to (u, v)
        if D > 1 {
            LCS(A[1..x], x, B[1..y], y)
            Output A[x+1..u]
            LCS(A[u+1..N], N-u, B[v+1..M], M-v)
        } else if M > N {
            Output A[1..N]
        } else {
            Output B[1..M]
        }
    }
}
*/
function printLines(opt: any, lines: any) {
    lines.forEach((line: any) => {
        console.log(opt, line);
    })
}
function printResult(A: any, B: any, x: any, y: any, u: any, v: any) {
    console.log('A: ', A);
    console.log('B: ', B);
    console.log('x: ', x, 'y: ', y, 'u: ', u, 'v: ', v);
    console.log('devided to');
    console.log(' - - - - - - - - - - - - - - - ');
    console.log('A1: ', A.slice(0, x));
    console.log('B1: ', B.slice(0, y));
    console.log(' - - - - - - - - - - - - - - - ');
    console.log('A2: ', A.slice(x, u + 1));
    console.log('B2: ', B.slice(y, v + 1));
    console.log(' - - - - - - - - - - - - - - - ');
    console.log('A3: ', A.slice(u + 1, A.length));
    console.log('B3: ', B.slice(v + 1, B.length));
    console.log('===============================');
}
function diff(A: any, B: any) {
    const N = A.length;
    const M = B.length;

    if (N === 0)
        printLines('+', B);
    else if (M === 0)
        printLines('-', A);
    else {
        const snake = FindMiddleSnake(A, N, B, M);
        const x = snake?.x, y = snake?.y, u = snake?.u! - 1, v = snake?.v! - 1;
        // printResult(A, B, x, y, u, v);
        if (x != null) {
            diff(A.slice(0, x), B.slice(0, y));
            printLines(' ', A.slice(x, u + 1));
            diff(A.slice(u + 1, N), B.slice(v + 1, M));
        } else { // 같은 문자열이 없(을 수도 있??)는 경우
            printLines('-', A);
            printLines('+', B);
        }
    }
}

export function run(file1Path: any, file2Path: any) {
    const file1 = loadFile(file1Path);
    const file2 = loadFile(file2Path);

    const A = file1!.split('\n');
    const B = file2!.split('\n');

    diff(A, B);
}