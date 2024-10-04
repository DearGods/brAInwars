<template>
  <div class="connect_button light_background flex align_center pointer" @click="clicked()">
    <div class="text_wrapper dark_background">
      <span class="light_text">
        CONNECT
      </span>
    </div>
    <Icon icon="wallet" class="icon" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Icon from '../Global/Icon.vue';

export default defineComponent({
  name: "ConnectButton",

  components: {
    Icon,
  },

  methods: {
    clicked() {
      this.$emit('clicked');
    }
  }
});
</script>

<style scoped lang="scss">
.connect_button {
  border-radius: 15px;
  width: 150px;
  height: 30px;
  position: relative;
  top: 1px;
  padding-left: 5px;
  transition: .3s scale linear, .3s box-shadow linear;

  &:hover {
    scale: 1.05;
    box-shadow: 0 0 8px 2px var(--lightBlueTrans);
  }

  .icon {
    position: absolute;
    right: 5px;
    top: -2px;
    font-size: 2.2em;
  }

  .text_wrapper {
    border-radius: 12px;
    padding: 3px 8px;
  }
}
</style>