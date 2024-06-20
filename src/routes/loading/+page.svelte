<script>
    // Your Svelte script code goes here
</script>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }

    .spinner {
        display: inline-block;
        width: 50px;
        height: 50px;
        border: 3px solid #ccc;
        border-radius: 50%;
        border-top-color: #333;
        animation: spin 1s ease-in-out infinite;

        @keyframes spin {
            0% {
                transform: rotate(0deg);
            }
            100% {
                transform: rotate(360deg);
            }
        }
    }
</style>

<div class="container">
    <div class="spinner">
    </div>
</div>