<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ref } from 'vue'
import ClipboardPopup from './components/ClipboardPopup.vue'

const ClipboardPopupRef = ref<InstanceType<typeof ClipboardPopup>>()

const showClipboardPopup = () => {
  if (ClipboardPopupRef.value) ClipboardPopupRef.value.showPopup()
}

const hideClipboardPopup = () => {
  if (ClipboardPopupRef.value) ClipboardPopupRef.value.hidePopup()
}
const toggleClipboardPopup = () => {
  if (ClipboardPopupRef.value) ClipboardPopupRef.value.togglePopup()
}
</script>

<template>
  <header>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />

    <div class="wrapper">
      <!-- <RouterView /> -->
      <button @click="showClipboardPopup">showClipboardPopup</button>
      <button @click="hideClipboardPopup">hideClipboardPopup</button>
      <button @click="toggleClipboardPopup">toggleClipboardPopup</button>
      <ClipboardPopup ref="ClipboardPopupRef" />
    </div>
  </header>
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
