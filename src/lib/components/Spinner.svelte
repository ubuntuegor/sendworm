<div class="loader"></div>

<style>
  @keyframes rotation {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .loader {
    width: 40px;
    height: 40px;
    border-top: solid 2px currentColor;
    border-right: solid 2px transparent;
    border-radius: 50%;
    animation: rotation 1s linear infinite;
  }
</style>
