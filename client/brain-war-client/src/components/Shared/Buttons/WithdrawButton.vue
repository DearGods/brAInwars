<template>
  <div class="withdraw_button light_background_trans flex align_center content_center pointer" @click="clicked()">
    <span class="white_text">
      Withdraw
    </span>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "WithdrawButton",

  methods: {
    clicked() {
      this.$emit('clicked');
    }
  }
});
</script>

<style scoped lang="scss">
.withdraw_button {
  border-radius: 3px;
  width: 100%;
  height: 30px;
  position: relative;
  top: 1px;
  padding-left: 5px;
  transition: .3s scale linear, .3s box-shadow linear;
  border: 2px solid var(--lightBlueTrans);

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