<template>
  <div class="logout_button light_background flex align_center pointer" @click="clicked()">
    <div class="text_wrapper dark_background">
      <span class="light_text">
        LOGOUT
      </span>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "LogoutButton",

  methods: {
    clicked() {
      this.$emit('clicked');
    }
  }
});
</script>

<style scoped lang="scss">
.logout_button {
  border-radius: 15px;
  height: 30px;
  position: relative;
  top: 1px;
  padding: 0px 3px;
  transition: .3s scale linear, .3s box-shadow linear;

  &:hover {
    scale: 1.05;
    box-shadow: 0 0 8px 2px var(--lightBlueTrans);
  }

  .text_wrapper {
    border-radius: 12px;
    padding: 3px 8px;
  }
}
</style>