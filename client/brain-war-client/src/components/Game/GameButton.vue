<template>
  <div
    class="game_button pointer"
    :class="{
      game_button_animation: animation,
      game_button_hidden: isHidden,
      game_button_disabled: disabled,
    }"
    @click="submit()"
  >
    <div class="game_button_darker" v-show="isHidden || disabled"></div>
    <h2 class="white_text">
      {{ text }}
    </h2>
    <img :src="imageSrc" alt="game button" class="game_button_image" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "GameButton",

  props: {
    animation: {
      type: Boolean,
      required: true,
    },

    text: {
      type: String,
      required: true,
    },

    isHidden: {
      type: Boolean,
      default: false,
    },

    disabled: {
      type: Boolean,
      default: false,
    },
  },

  data() {
    return {};
  },

  computed: {
    imageSrc(): string {
      return "https://cdn.brainwars.win/public%2Fimages%2Fgame%2Fbutton.webp";
    },
  },

  methods: {
    submit() {
      this.$store.dispatch("playAudio", "click-2");
      this.$emit("submit");
    },
  },
});
</script>

<style scoped lang="scss">
.game_button {
  position: relative;
  width: 100%;
  height: 100%;
  top: 0%;
  transition:
    0.7s top ease-in,
    0.3s scale linear;

  &:hover {
    scale: 1.05;
  }

  h2 {
    position: absolute;
    text-align: center;
    left: 0;
    right: 0;
    font-size: 3em;
    z-index: 99;
    top: 35%;

    @media only screen and (max-width: 600px) {
      top: 25%;
    }
  }

  img {
    width: 100%;
    z-index: 0;
  }
}

.game_button_hidden {
  pointer-events: none;
  top: 30%;
  opacity: 0.8;
}

.game_button_disabled {
  pointer-events: none;
  opacity: 0.8;
}

.game_button_darker {
  height: 100%;
  width: 100%;
  background-color: #000;
  border-radius: 50%;
  position: absolute;
  opacity: 0.4;
  z-index: 101;
}

.game_button_animation {
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0% {
    transform: scale(1);
  }

  30% {
    transform: scale(1.05);
  }

  40% {
    transform: scale(1);
  }

  50% {
    transform: scale(1.05);
  }

  100% {
    transform: scale(1);
  }
}
</style>
