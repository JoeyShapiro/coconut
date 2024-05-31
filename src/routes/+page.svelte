<div bind:clientHeight={h}>
    <h1>Welcome to SvelteKit</h1>
    <p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
    <input type="range" min="1" max="100" value="{0}" class="volume" id="volume">
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
        id: number;
        name: string;
        pos: Pos;
        isCurrent: boolean;
        amp: number;
        theta: number;
        constructor(id: number, name: string, pos: Pos, isCurrent: boolean = false, amp: number = 0, theta: number = 0) {
            this.id = id;
            this.name = name;
            this.pos = pos;
            this.isCurrent = isCurrent;
            this.amp = amp;
            this.theta = theta;
        }
    }

    let h = 0;
    let users: User[];
    let talkers: number[] = [];
    let selected = -1; // TODO make this 2d
    let selector: Pos[] = [];
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
        cur = users.findIndex((user) => user.isCurrent);
        for (const user of users) {
            let distance = Math.sqrt(Math.pow(users[cur].pos.x - user.pos.x, 2) + Math.pow(users[cur].pos.y - user.pos.y, 2));
            let theta = Math.atan2(user.pos.y - users[cur].pos.y, user.pos.x - users[cur].pos.x);

            user.amp = 1 - (distance / window.innerHeight); // TODO shrug
            user.theta = theta;

            invoke('user_update', { id: user.id, user: user })
                .then(updateResponse)
                .catch(updateResponse);
        }

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
            selector = [
                new Pos( e.offsetX, e.offsetY )
            ];
            isSelecting = true;
        }, false);
        canvas.addEventListener("mouseup", function (e) {
            if (selected !== -1) {
                users[selected].pos = new Pos(e.offsetX, e.offsetY);
                console.log(`moved users[${selected}] to (${e.offsetX}, ${e.offsetY})`);

                console.log(`users[${selected}]`, users[selected]);
                let distance = Math.sqrt(Math.pow(users[cur].pos.x - users[selected].pos.x, 2) + Math.pow(users[cur].pos.y - users[selected].pos.y, 2));
                let theta = Math.atan2(users[selected].pos.y - users[cur].pos.y, users[selected].pos.x - users[cur].pos.x);

                users[selected].amp = 1 - (distance / window.innerHeight); // TODO shrug
                users[selected].theta = theta;

                invoke('user_update', { id: users[selected].id, user: users[selected] })
                    .then(updateResponse)
                    .catch(updateResponse);

                selected = -1;
            }
            selector[1] = new Pos( e.offsetX, e.offsetY );
            isSelecting = false;

            // check if any users are in the selector
            for (let i = 0; i < users.length; i++) {
                if (users[i].pos.x > selector[0].x && users[i].pos.x < selector[1].x &&
                    users[i].pos.y > selector[0].y && users[i].pos.y < selector[1].y) {
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
                selector[1] = new Pos( e.offsetX, e.offsetY );
            }
        }, false);
    
        function updateResponse(response: any) {
            console.log(response);
        }

        const volume = document.querySelector('#volume') as HTMLInputElement;
        volume.addEventListener('input', () => {
            invoke('set_volume', {
                    value: parseFloat(volume.value)
                })
                .then(updateResponse)
                .catch(updateResponse)
        });

        setInterval(() => {
            redraw();
        }, 100);

        setInterval(async () => {
            talkers = await invoke('get_talkers').then((response) => {
                return response as number[];
            }).catch((error) => {
                console.error(error);
                return [] as number[];
            });
        }, 1000);

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
                ctx.rect(selector[0].x, selector[0].y, selector[1].x - selector[0].x, selector[1].y - selector[0].y);
                ctx.stroke();
            }

            for (let i = 0; i < users.length; i++) {
                const user = users[i];

                ctx.beginPath();
                ctx.arc(user.pos.x, user.pos.y, 10, 0, 2 * Math.PI, false);
                if (user.isCurrent) { // TODO maybe just use cur
                    ctx.fillStyle = 'blue';
                } else {
                    ctx.fillStyle = 'green';
                }
                ctx.fill();

                if (talkers.includes(user.id)) {
                    ctx.lineWidth = 2;
                    ctx.strokeStyle = '#990000';
                } else {
                    ctx.lineWidth = 1;
                    ctx.strokeStyle = '#003300';
                }
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
                    ctx.fillText(`${user.name} (${Math.round(user.amp*100)}%)`, user.pos.x + 10, user.pos.y + 10);
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