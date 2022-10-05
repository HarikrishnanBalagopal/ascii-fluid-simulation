const COLUMN_LEVEL = `#  include<stdio.h>//  .IOCCC                                         Fluid-  #
#  include <unistd.h>  //2012                                         _Sim!_  #
#  include<complex.h>  //||||                     ,____.              IOCCC-  #
#  define              h for(                     x=011;              2012/*  #
#  */-1>x              ++;)b[                     x]//-'              winner  #
#  define              f(p,e)                                         for(/*  #
#  */p=a;              e,p<r;                                        p+=5)//  #
#  define              z(e,i)                                        f(p,p/*  #
## */[i]=e)f(q,w=cabs  (d=*p-  *q)/2-     1)if(0  <(x=1-      w))p[i]+=w*/// ##
   double complex a [  97687]  ,*p,*q     ,*r=a,  w=0,d;    int x,y;char b/* ##
## */[6856]="\x1b[2J"  "\x1b"  "[1;1H     ", *o=  b, *t;   int main   (){/** ##
## */for(              ;0<(x=  getc (     stdin)  );)w=x  >10?32<     x?4[/* ##
## */*r++              =w,r]=  w+1,*r     =r[5]=  x==35,  r+=9:0      ,w-I/* ##
## */:(x=              w+2);;  for(;;     puts(o  ),o=b+  4){z(p      [1]*/* ##
## */9,2)              w;z(G,  3)(d*(     3-p[2]  -q[2])  *P+p[4      ]*V-/* ##
## */q[4]              *V)/p[  2];h=0     ;f(p,(  t=b+10  +(x=*p      *I)+/* ##
## */80*(              y=*p/2  ),*p+=p    [4]+=p  [3]/10  *!p[1])     )x=0/* ##
## */ <=x              &&x<79   &&0<=y&&y<23?1[1  [*t|=8   ,t]|=4,t+=80]=1/* ##
## */, *t              |=2:0;    h=" '\`-.|//,\\"  "|\\_"    "\\/\x23\n"[x/** ##
## */%80-              9?x[b]      :16];;usleep(  12321)      ;}return 0;}/* ##
####                                                                       ####
###############################################################################
**###########################################################################*/`;

async function main() {
    const CONSOLE_WIDTH = 80;
    const CONSOLE_HEIGHT = 24;
    const N_SIZE = CONSOLE_WIDTH * CONSOLE_HEIGHT + 1;

    console.log('start');
    console.log(COLUMN_LEVEL);
    console.log('COLUMN_LEVEL.length', COLUMN_LEVEL.length, 'N_SIZE', N_SIZE);
    if(COLUMN_LEVEL.length > N_SIZE) return console.error('COLUMN_LEVEL is too large');

    let PAUSED = false;

    const res = await fetch('assets/wasm/test_fluid.wasm', { headers: { 'Accept': 'application/wasm' } });
    if (!res.ok) return console.error('failed to fetch the web-assembly module. status:', res.statusText);
    const moduleBytes = await res.arrayBuffer();
    const importObject = {};
    const module = await WebAssembly.instantiate(moduleBytes, importObject);
    console.log('module', module);

    // ------------------------------------------------------

    const input_address = module.instance.exports.allocate_memory_for_file(COLUMN_LEVEL.length);
    console.log('input_address', input_address);
    const input_mem = new Uint8Array(module.instance.exports.memory.buffer, input_address, COLUMN_LEVEL.length);
    console.log('input_mem before', input_mem);
    for(let i = 0; i < COLUMN_LEVEL.length; i++) {
        input_mem[i] = COLUMN_LEVEL.charCodeAt(i);
    }
    console.log('input_mem after', input_mem);

    // module.instance.exports.initialize_global(input_address);
    const total_particles = module.instance.exports.initialize_global(input_address);
    console.log('total_particles', total_particles);

    let last_t = null;
    const TIME_STEP = 1;
    const fluid_output = document.querySelector('#fluid-output');
    const decoder = new TextDecoder();

    const address = module.instance.exports.get_address_global();
    console.log('address', address);
    const mem = new Uint8Array(module.instance.exports.memory.buffer, address, N_SIZE);
    console.log('mem', mem);
    function draw(t) {
        if(PAUSED)return;
        requestAnimationFrame(draw);
        if (!last_t) last_t = t;
        if (t - last_t < TIME_STEP) return;
        last_t = t;
        module.instance.exports.step_global(total_particles);
        // console.log(mem);
        fluid_output.textContent = decoder.decode(mem);
    }
    requestAnimationFrame(draw);

    const play_pause_button = document.querySelector('#play-pause-button');
    play_pause_button.addEventListener('click', () => {
        if (PAUSED) {
            PAUSED = false;
            play_pause_button.textContent = '⏸ Pause';
            if(fluid_output.getAttribute('contenteditable')) {
                fluid_output.removeAttribute('contenteditable');
                const new_level = fluid_output.textContent;
                console.log('new_level', new_level, new_level.length);

                const input_address = module.instance.exports.allocate_memory_for_file(COLUMN_LEVEL.length);
                console.log('input_address', input_address);
                const input_mem = new Uint8Array(module.instance.exports.memory.buffer, input_address, COLUMN_LEVEL.length);
                console.log('input_mem before', input_mem);
                for(let i = 0; i < new_level.length; i++) {
                    input_mem[i] = new_level.charCodeAt(i);
                }
                console.log('input_mem after', input_mem);
                for(let i = 0; i < mem.length; i++) {
                    mem[i] = 0;
                }
                // module.instance.exports.initialize_global(input_address);
                const total_particles = module.instance.exports.initialize_global(input_address);
                console.log('total_particles', total_particles);                     
            }
            requestAnimationFrame(draw);
        } else {
            PAUSED = true;
            play_pause_button.textContent = '▶️ Play';
        }
    });
    const edit_button = document.querySelector('#edit-button');
    edit_button.addEventListener('click', () => {
        fluid_output.setAttribute('contenteditable', 'true');
    });

    console.log('done');
}

main().catch(console.error);
