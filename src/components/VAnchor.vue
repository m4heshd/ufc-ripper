<template>
  <a
      class="v-anchor"
      target="_blank"
      rel="noopener noreferrer"
  >
    <slot></slot>
  </a>
</template>

<style lang="scss">
.v-anchor {
  color: var(--primary);
  text-decoration: underline;
}
</style>