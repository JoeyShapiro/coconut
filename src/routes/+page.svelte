<div bind:clientHeight={h}>
    <h1>Welcome to SvelteKit</h1>
    <p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
    <button id="increment-btn">Increment counter</button>
    <input type="range" min="1" max="100" value="50" class="slider" id="myRange">
</div>

<div class="container">
    <canvas id="myCanvas" width="{window.innerWidth}" height="{window.innerHeight-h}"></canvas>
</div>

<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    class Pos {
        x: number;
        y: number;
        constructor(x: number, y: number) {
            this.x = x;
            this.y = y;
        }
    }

    class User {
        name: string;
        pos: Pos;
        isCurrent: boolean; // TODO this
        constructor(name: string, pos: Pos, isCurrent: boolean = false) {
            this.name = name;
            this.pos = pos;
            this.isCurrent = isCurrent;
        }
    }

    let h = 0;
    let users: User[];
    let selected = -1; // TODO make this 2d
    let selector: any[][] = [];
    let isSelecting: boolean = false;
    let cur = 2; // good enough for now; better than hardcoding
    onMount(async () => {
        // console.log(window.innerHeight, window.innerWidth)

        // window.addEventListener("resize", () => {
        //     redraw();
        // });
        users = await invoke('get_users').then((response) => {
            console.log(response);
            return response as User[];
        }).catch((error) => {
            console.error(error);
            return [] as User[];
        });

        const canvas = document.getElementById('myCanvas');
        if (!canvas) {
            return;
        }
        canvas.addEventListener("mousedown", function (e) {
            for (let i = 0; i < users.length; i++) {
                if (Math.sqrt(Math.pow(e.offsetX - users[i].pos.x, 2) + Math.pow(e.offsetY - users[i].pos.y, 2)) < 10) {
                    console.log(`clicked on users[${i}]`, users[i].name);
                    selected = i;
                    return;
                }
            }
            selector[0] = [ e.offsetX, e.offsetY ];
            selector[1] = [];
            isSelecting = true;
        }, false);
        canvas.addEventListener("mouseup", function (e) {
            if (selected !== -1) {
                users[selected].pos = new Pos(e.offsetX, e.offsetY);
                console.log(`moved users[${selected}] to (${e.offsetX}, ${e.offsetY})`);
                selected = -1;
            }
            selector[1] = [ e.offsetX, e.offsetY ];
            isSelecting = false;

            // check if any users are in the selector
            for (let i = 0; i < users.length; i++) {
                if (users[i].pos.x > selector[0][0] && users[i].pos.x < selector[1][0] &&
                    users[i].pos.y > selector[0][1] && users[i].pos.y < selector[1][1]) {
                    console.log(`selected users[${i}]`, users[i].name);
                }
            }
        }, false);
        // TODO would like to do in main loop, but i dont have it
        canvas.addEventListener("mousemove", function (e) {
            if (selected !== -1) {
                users[selected].pos = new Pos(e.offsetX, e.offsetY);
            }
            // change the style of the cursor
            canvas.style.cursor = "default";
            for (let i = 0; i < users.length; i++) {
                if (Math.sqrt(Math.pow(e.offsetX - users[i].pos.x, 2) + Math.pow(e.offsetY - users[i].pos.y, 2)) < 10) {
                    canvas.style.cursor = "pointer";
                    break;
                }
            }

            // set the second point of the selector
            if (isSelecting) {
                selector[1] = [ e.offsetX, e.offsetY ];
            }
        }, false);
    
        function updateResponse(response: any) {
            console.log(response);
        }

        const incrementBtn = document.querySelector('#increment-btn') as HTMLButtonElement;
        incrementBtn.addEventListener('click', () => {
            invoke('set_amplifier', {
                    value: 5.0
                })
                .then(updateResponse)
                .catch(updateResponse)
        });

        setInterval(() => {
            redraw();
        }, 100);

        function redraw() {
            const canvas = document.getElementById('myCanvas');
            // @ts-ignore
            const ctx = canvas.getContext('2d');

            // clear the canvas
            // @ts-ignore
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            // draw the selector
            if (isSelecting && selector.length === 2) {
                ctx.beginPath();
                ctx.rect(selector[0][0], selector[0][1], selector[1][0] - selector[0][0], selector[1][1] - selector[0][1]);
                ctx.stroke();
            }

            for (let i = 0; i < users.length; i++) {
                const user = users[i];

                ctx.beginPath();
                ctx.arc(user.pos.x, user.pos.y, 10, 0, 2 * Math.PI, false);
                if (user.isCurrent) {
                    ctx.fillStyle = 'blue';
                } else {
                    ctx.fillStyle = 'green';
                }
                ctx.fill();
                ctx.lineWidth = 1;
                ctx.strokeStyle = '#003300';
                ctx.stroke();

                let distance = Math.sqrt(Math.pow(users[cur].pos.x - user.pos.x, 2) + Math.pow(users[cur].pos.y - user.pos.y, 2));
                // round to 2 decimal places
                distance = Math.round(distance * 100) / 100;

                // write the distance next to the user
                ctx.font = "12px Arial";
                ctx.fillStyle = "black";
                // TODO do seomthing with the distance
                // TODO radius based on window or something
                if (user.isCurrent) { // can now use the pos rather than reget it
                    ctx.fillText(`(${user.pos.x}, ${user.pos.y})`, user.pos.x + 10, user.pos.y + 10);
                } else {
                    ctx.fillText(distance, user.pos.x + 10, user.pos.y + 10);
                }
            }
        }
    });
</script>

<style>
.container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    border: 1px solid red;
}

canvas {
    border: 1px solid black;
    max-width: 100%;
    max-height: 100%;
    width: 100%;
    height: 100%;
}
</style>