#![feature(test,asm)]
extern crate test;

use test::Bencher;

static test_len:usize = 10;
static test_linear_key:u32 = 5;
static test_binary_key:u32 = 3;

/*
////////////// Think Pad,R400 Core2 Duo CPU P8600 (2.4G 2.4G) x64 win7 RAM 8G
test_len = 1000;
linear_key = 500;
binary_key = 200;
running 6 tests
test bench_binary_search              ... bench:          42 ns/iter (+/- 8)
test bench_binary_search_unsafe       ... bench:          41 ns/iter (+/- 2)
test bench_linear_search              ... bench:         637 ns/iter (+/- 14)
test bench_linear_sentinel            ... bench:         432 ns/iter (+/- 21)
test bench_linear_sentinel_unrolling4 ... bench:         431 ns/iter (+/- 45)
test bench_linear_sentinel_unrolling8 ... bench:         420 ns/iter (+/- 19)

test_len = 100;
linear_key = 50;
binary_key = 20;

running 6 tests
test bench_binary_search              ... bench:          27 ns/iter (+/- 3)
test bench_binary_search_unsafe       ... bench:          25 ns/iter (+/- 1)
test bench_linear_search              ... bench:          65 ns/iter (+/- 10)
test bench_linear_sentinel            ... bench:          44 ns/iter (+/- 16)
test bench_linear_sentinel_unrolling4 ... bench:          44 ns/iter (+/- 3)
test bench_linear_sentinel_unrolling8 ... bench:          43 ns/iter (+/- 5)


test_len = 50;
linear_key = 20;
binary_key = 10; //use key from  to 10
running 6 tests
test bench_binary_search              ... bench:          25 ns/iter (+/- 9)
test bench_binary_search_unsafe       ... bench:          21 ns/iter (+/- 0)
test bench_linear_search              ... bench:          27 ns/iter (+/- 2)
test bench_linear_sentinel            ... bench:          18 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:          19 ns/iter (+/- 1)
test bench_linear_sentinel_unrolling8 ... bench:          19 ns/iter (+/- 1)


test_len = 10;
linear_key = 5;
binary_key = 3;
running 6 tests
test bench_binary_search              ... bench:          11 ns/iter (+/- 0)
test bench_binary_search_unsafe       ... bench:           9 ns/iter (+/- 0)
test bench_linear_search              ... bench:           8 ns/iter (+/- 0)
test bench_linear_sentinel            ... bench:           5 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:           6 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling8 ... bench:           6 ns/iter (+/- 0)
*/

////////////// i3 cpu/////////////////////////////////////
/*
test_len = 1000;
linear_key = 500;
binary_key = 200;
running 6 tests
test bench_binary_search              ... bench:          23 ns/iter (+/- 0)
test bench_binary_search_unsafe       ... bench:          21 ns/iter (+/- 1)
test bench_linear_search              ... bench:         293 ns/iter (+/- 22)
test bench_linear_sentinel            ... bench:         229 ns/iter (+/- 8)
test bench_linear_sentinel_unrolling4 ... bench:         174 ns/iter (+/- 13)
test bench_linear_sentinel_unrolling8 ... bench:         178 ns/iter (+/- 6)

test_len = 100;
linear_key = 50;
binary_key = 20;
running 4 tests
test bench_binary_search              ... bench:          13 ns/iter (+/- 1)
test bench_linear_search              ... bench:          35 ns/iter (+/- 1)
test bench_linear_sentinel            ... bench:          21 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:          19 ns/iter (+/- 0)


test_len = 50;
linear_key = 20;
binary_key = 5; //use key from  to 10
running 4 tests
test bench_binary_search              ... bench:          11 ns/iter (+/- 0)
test bench_linear_search              ... bench:          12 ns/iter (+/- 4)
test bench_linear_sentinel            ... bench:          10 ns/iter (+/- 1)
test bench_linear_sentinel_unrolling4 ... bench:           9 ns/iter (+/- 0)


test_len = 30;
linear_key = 10;
binary_key = 5;
running 4 tests
test bench_binary_search              ... bench:           8 ns/iter (+/- 0)
test bench_linear_search              ... bench:           9 ns/iter (+/- 0)
test bench_linear_sentinel            ... bench:           8 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:           6 ns/iter (+/- 0)


test_len = 30;
linear_key = 10;
binary_key = 5;
running 5 tests
test bench_binary_search              ... bench:           9 ns/iter (+/- 1)
test bench_linear_search              ... bench:           9 ns/iter (+/- 1)
test bench_linear_sentinel            ... bench:           8 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:           6 ns/iter (+/- 1)
test bench_linear_sentinel_unrolling8 ... bench:           5 ns/iter (+/- 0)

test_len = 10;
linear_key = 5;
binary_key = 3;
running 5 tests
test bench_binary_search              ... bench:           4 ns/iter (+/- 1)
test bench_linear_search              ... bench:           3 ns/iter (+/- 1)
test bench_linear_sentinel            ... bench:           3 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling4 ... bench:           2 ns/iter (+/- 0)
test bench_linear_sentinel_unrolling8 ... bench:           2 ns/iter (+/- 0)
*/

/*
conclusion:
1. binary search is efficient enough almost all cases ,when array size is small,the search time can be ingored,so we don't need to compare them
2. bound check takes nearly 10% time cost (23-21)/23 = 8.7%
3. unrolling is more important than sentinel in linear search in i3 than P8600
*/

#[bench]
fn bench_linear_search(b: &mut Bencher) {
    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
    	v.push(i as u32);
    }

    b.iter(|| {
    	for i in (0..test_len) {
    		if v[i] >= test_linear_key {
                assert!(v[i] as u32 == test_linear_key);
    			break;
    		}
    	}
    });
}

#[bench]
fn bench_linear_sentinel(b: &mut Bencher) {
    let mut v:Vec<u32> = Vec::with_capacity(test_len+1);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    //push a sentinel as large as possible
    v.push(1<< 31);

    b.iter(|| {
        let mut i = 0;
        loop {
            if v[i] >= test_linear_key {
                break;
            }
            i+=1;
        }
        assert!(v[i] as u32 == test_linear_key);
    });
}

//why unrolling not work?
#[bench]
fn bench_linear_sentinel_unrolling4(b: &mut Bencher) {
    let mut len = test_len;
    let mut ext_len = ((len + 3) /4) * 4; //ceilling
    let mut v:Vec<u32> = Vec::with_capacity(ext_len as usize);
    for i in (0..len) {
        v.push(i as u32);
    }

    //push sentinels as large as possible
    for i in (len..ext_len){
        v.push(1<< 31);
    }

    b.iter(|| {
        let mut i = 0;
        let mut pos = 0;
        loop {
            //unrolling 4 
            if v[i] >= test_linear_key {
                pos = i;
                break;
            }

            if v[i+1] >= test_linear_key {
                pos = i+1;
                break;
            }

            if v[i+2] >= test_linear_key {
                pos = i+2;
                break;
            }

            if v[i+3] >= test_linear_key {
                pos = i+3;
                break;
            }

            i+=4;
        }
        assert!(v[pos] as u32 == test_linear_key);
    });
}

#[bench]
fn bench_linear_sentinel_unrolling8(b: &mut Bencher) {
    let mut len = test_len;
    let mut ext_len = ((len + 3) /8) * 8; //ceilling
    let mut v:Vec<u32> = Vec::with_capacity(ext_len as usize);
    for i in (0..len) {
        v.push(i as u32);
    }

    //push sentinels as large as possible
    for i in (len..ext_len){
        v.push(1<< 31);
    }

    b.iter(|| {
        let mut i = 0;
        let mut pos = 0;
        loop {
            //unrolling 4 
            if v[i] >= test_linear_key {
                pos = i;
                break;
            }

            if v[i+1] >= test_linear_key {
                pos = i+1;
                break;
            }

            if v[i+2] >= test_linear_key {
                pos = i+2;
                break;
            }

            if v[i+3] >= test_linear_key {
                pos = i+3;
                break;
            }

            if v[i+4] >= test_linear_key {
                pos = i+4;
                break;
            }

            if v[i+5] >= test_linear_key {
                pos = i+5;
                break;
            }

            if v[i+6] >= test_linear_key {
                pos = i+6;
                break;
            }

            if v[i+7] >= test_linear_key {
                pos = i+7;
                break;
            }

            i+=8;
        }
        assert!(v[pos] as u32 == test_linear_key);
    });
}

#[bench]
fn bench_binary_search(b: &mut Bencher) {

    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    b.iter(|| {
        let mut left = 0;
        let mut right = test_len-1;
        let mut mid = 0;
        while left < right {
            mid = (left + right) >> 1;
            assert!(mid < right);
            //key > v[mid],so left = mid + 1
            if v[mid] < test_binary_key {
                left = mid + 1;
            }else{
                //here means v[mid] >= key,v[mid] possibly equal key,so right = mid but not mid - 1
                right = mid;
            }
        }
        assert!((left == right) && (v[left] == test_binary_key));
    });
}

// to avoid bound check
#[bench]
fn bench_binary_search_unsafe(b: &mut Bencher) {

    let mut v:Vec<u32> = Vec::with_capacity(test_len);
    for i in (0..test_len) {
        v.push(i as u32);
    }

    let pt = &v[0] as *const u32;

    b.iter(|| {
        let mut left = 0;
        let mut right = test_len-1;
        let mut mid = 0;
        while left < right {
            mid = (left + right) >> 1;
            assert!(mid < right);
            //key > v[mid],so left = mid + 1
            //if v[mid] < test_binary_key {
            if unsafe{*pt.offset(mid as isize)} < test_binary_key {
                left = mid + 1;
            }else{
                //here means v[mid] >= key,v[mid] possibly equal key,so right = mid but not mid - 1
                right = mid;
            }
        }
        assert!((left == right) && (v[left] == test_binary_key));
    });
}


//how to use cmov in rust?the following code didn't work
// #[bench]
// fn bench_binary_search_cmov(b: &mut Bencher) {

//     let mut v:Vec<u32> = Vec::with_capacity(test_len);
//     for i in (0..test_len) {
//         v.push(i as u32);
//     }

//     b.iter(|| {
//         let mut left:u32 = 0;
//         let mut right:u32 = (test_len-1) as u32;
//         let mut mid:u32 = 0;
//         while left < right {
//             mid = (left + right) >> 1;
//             assert!(mid < right);
//             unsafe{
//                 // asm! ("cmpl %3, %2 cmovg %4, %0 cmovle %5, %1"
//                 //      : "+r" (left), "+r" (right)
//                 //      : "r" (test_binary_key as u32), "g" (v[mid as usize]), "g" (mid + 1), "g" (mid)
//                 //      );
//                 let k = test_binary_key as u32;
//                 let vv = v[mid as usize];
//                 asm!("cmpl $1, $0"
//                     :
//                     :"r"(k),"g"(vv)
//                     );

//                 asm!("cmovg $1, $0"
//                  : "=r"(left)
//                  : "g"(mid + 1)
//                  );

//                 asm!("cmovle $1, $0"
//                  : "=r"(right)
//                  : "g"(mid)
//                  );
//             }
            
//         }
//         assert!((left == right) && (v[left as usize] == test_binary_key));
//     });
// }


// fn add(a: i32, b: i32) -> i32 {
//     let c: i32;
//     unsafe {
//         asm!("add $2, $0"
//              : "=r"(c)
//              : "0"(a), "r"(b)
//              );
//     }
//     c
// }