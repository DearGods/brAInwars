<template>
  <template>
  </template>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "LogoutPage",
  
  created() {
    this.$store.dispatch('logout');
    this.$router.back()
  },
});
</script>
