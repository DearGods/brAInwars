<template>
  <img :src="logoSrc" alt="Logo" class="h100 w100 pointer" @click="submit()" />
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "AppLogo",

  computed: {
    logoSrc() {
      return "https://cdn.brainwars.win/assets%2Fimages%2FLogo.png";
    },
  },

  methods: {
    submit() {
      this.$emit("submit");
    },
  },
});
</script>

<style scoped>
img {
  object-fit: cover;
}
</style>
