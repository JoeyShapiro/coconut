<div bind:clientHeight={h}>
    <h1>Welcome to SvelteKit</h1>
    <p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
</div>

<div class="container">
    <canvas id="myCanvas" width="{window.innerWidth}" height="{window.innerHeight-h}"></canvas>
</div>

<script>
  import { onMount } from 'svelte';
    let h = 0;
    /**
	 * @type {any[]}
	 */
    let circles = [];
    onMount(() => {
        console.log(window.innerHeight, window.innerWidth)

        circles[0] = {
            x: 100,
            y: 100,
            r: 10,
            distance: 0
        };
        circles[1] = {
            x: 200,
            y: 200,
            r: 10,
            distance: 0
        };

        window.addEventListener("resize", () => {
            redraw();
        });
        redraw();

        function redraw() {
            const canvas = document.getElementById('myCanvas');
            // @ts-ignore
            const ctx = canvas.getContext('2d');
            ctx.fillStyle = 'rgb(200, 0, 0)';
            ctx.fillRect(10, 10, 55, 50);

            ctx.arc(window.innerWidth/2, window.innerHeight/2, 10, 0, 2 * Math.PI, false);
            ctx.fillStyle = 'blue';
            ctx.fill();
            ctx.lineWidth = 1;
            ctx.strokeStyle = '#003300';
            ctx.stroke();

            ctx.font = "12px Arial";
            ctx.fillStyle = "black";
            ctx.fillText(`(${window.innerWidth/2}, ${window.innerHeight/2})`, window.innerWidth/2 + 10, window.innerHeight/2 + 10);

            for (const key in circles) {
                if (Object.hasOwnProperty.call(circles, key)) {
                    const circle = circles[key];
                    ctx.beginPath();
                    ctx.arc(circle['x'], circle['y'], circle['r'], 0, 2 * Math.PI, false);
                    ctx.fillStyle = 'green';
                    ctx.fill();
                    ctx.lineWidth = 1;
                    ctx.strokeStyle = '#003300';
                    ctx.stroke();

                    let distance = Math.sqrt(Math.pow(window.innerWidth/2 - circle.x, 2) + Math.pow(window.innerHeight/2 - circle.y, 2));
                    console.log(distance);
                    // round to 2 decimal places
                    distance = Math.round(distance * 100) / 100;

                    // write the distance next to the circle
                    ctx.font = "12px Arial";
                    ctx.fillStyle = "black";
                    ctx.fillText(distance, circle.x + circle.r, circle.y + circle.r);
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