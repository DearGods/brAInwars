<template>
  <span 
    class="material-symbols-outlined icon"
    :title="title"
    :class="{
      'pointer': pointer,
      'dark_text': !dark,
      'light_text': dark,
      'small_icon': small,
      'medium_icon': medium || (!small && !large),
      'large_icon': large,
    }"
    :style="color ? `color: ${color} !important` : ''"
    @click="clicked()"
  >
    {{ icon }}
  </span>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "IconItem",

  props: {
    icon: {
      type: String,
      required: true
    },
    
    title: {
      type: String,
      default: ''
    },
    
    color: {
      type: String,
      required: false,
    },
    
    pointer: {
      type: Boolean,
      default: true
    },
    
    dark: {
      type: Boolean,
      default: false
    },

    small: {
      type: Boolean,
      default: false
    },

    medium: {
      type: Boolean,
      default: false
    },

    large: {
      type: Boolean,
      default: false
    }

  },

  methods: {
    clicked() {
      this.$emit('clicked');
    }
  }
});
</script>

<style scoped lang="scss">
.icon {
  height: 25px;
  position: relative;
  z-index: 99;
}

.small_icon {
  font-size: 1.5em;
}

.medium_icon {
  font-size: 2.5em;
}

.large_icon {
  font-size: 3.5em;
}

</style>
