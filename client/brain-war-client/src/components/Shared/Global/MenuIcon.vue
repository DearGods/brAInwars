<template>
  <div
    class="menu_icon"
    :class="{
      open: isOpen,
    }"
    @click="clicked()"
  >
    <span class="light_background"></span>
    <span class="light_background"></span>
    <span class="light_background"></span>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "MenuIcon",

  computed: {
    isOpen(): boolean {
      return this.$store.getters.isMenuOpen;
    },
  },

  methods: {
    clicked() {
      this.$store.dispatch("playAudio", 'click');
      this.$store.dispatch('toggleMenuState');
    }
  }
});
</script>

<style scoped lang="scss">
.menu_icon {
  width: 35px;
  height: 25px;
  position: relative;
  z-index: 101;
  -webkit-transform: rotate(0deg);
  -moz-transform: rotate(0deg);
  -o-transform: rotate(0deg);
  transform: rotate(0deg);
  -webkit-transition: 0.5s ease-in-out;
  -moz-transition: 0.5s ease-in-out;
  -o-transition: 0.5s ease-in-out;
  transition: 0.5s ease-in-out;
  cursor: pointer;

  span {
    display: block;
    position: absolute;
    height: 5px;
    width: 100%;
    border-radius: 9px;
    opacity: 1;
    left: 0;
    -webkit-transform: rotate(0deg);
    -moz-transform: rotate(0deg);
    -o-transform: rotate(0deg);
    transform: rotate(0deg);
    -webkit-transition: 0.25s ease-in-out;
    -moz-transition: 0.25s ease-in-out;
    -o-transition: 0.25s ease-in-out;
    transition: 0.25s ease-in-out;

    &:nth-child(1) {
      top: 0px;
    }

    &:nth-child(2) {
      top: 10px;
    }

    &:nth-child(3) {
      top: 20px;
    }
  }

  &.open span:nth-child(1) {
    top: 10px;
    -webkit-transform: rotate(135deg);
    -moz-transform: rotate(135deg);
    -o-transform: rotate(135deg);
    transform: rotate(135deg);
  }

  &.open span:nth-child(2) {
    opacity: 0;
    left: -60px;
  }

  &.open span:nth-child(3) {
    top: 10px;
    -webkit-transform: rotate(-135deg);
    -moz-transform: rotate(-135deg);
    -o-transform: rotate(-135deg);
    transform: rotate(-135deg);
  }
}
</style>
