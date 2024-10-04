<template>
  <div></div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "AudioPlayer",

  methods: {
    togglePlay(audioName: string) {
      const audio = ref(new Audio());
      audio.value.src = `https://cdn.brainwars.win/public%2Fassets%2Faudios%2F${audioName}.mp3`;
      audio.value.play();
      audio.value.volume = 0.5;
      this.$store.dispatch("playAudio", null);
    },
  },

  watch: {
    audioFile(audioName) {
      if (!audioName) {
        return;
      }

      this.togglePlay(audioName);
    },
  },

  computed: {
    audioFile() {
      return this.$store.getters.getPlayingAudioFile;
    },
  },
});
</script>
