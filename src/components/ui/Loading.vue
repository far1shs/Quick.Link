<template>
  <div class="loading"></div>
</template>

<style scoped>
.loading {
  width: 24px;
  height: 24px;
  border: 2px solid var(--color-foreground);
  border-top-color: transparent;
  border-radius: 100%;

  animation: circle infinite 0.75s linear;
}

@keyframes circle {
  0% {
    transform: rotate(0);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>