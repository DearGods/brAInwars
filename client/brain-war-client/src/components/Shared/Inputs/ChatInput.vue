<template>
  <div class="chat_input h100">
    <BaseInput
      ref="baseInput"
      rightIcon
      rightIconAction
      :rightIconText="rightIconText"
      @rightIconClicked="rightIconClicked()"
      @valueUpdated="valueUpdated"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BaseInput from "./BaseInput.vue";

export default defineComponent({
  name: "ChatInput",

  components: { BaseInput },

  data() {
    return {
      rightIconText: "send",
    };
  },

  mounted() {
    this.scrollDownToLatestMessages();
  },

  watch: {
    chatMessageText() {
      if (!this.chatMessageText) {
        (this.$refs as any).baseInput.setValue("");
      }
    },
  },

  computed: {
    chatMessageText() {
      return this.$store.getters.chatMessageText;
    },
  },

  methods: {
    scrollDownToLatestMessages() {
      const messages = this.$refs.messages as HTMLDivElement;
      if (!messages) {
        return;
      }

      messages.scrollTop = messages.scrollHeight;
    },

    rightIconClicked() {
      this.$emit("rightIconClicked");
    },

    valueUpdated(value: string) {
      this.$store.dispatch("updateChatMessageText", value);
    },
  },
});
</script>

<style scoped lang="scss">
.game_chat {
  width: 100%;
  height: 100%;

  .game_chat_messages {
    overflow-y: auto;
    height: 84%;
    width: 90%;
    margin-top: 7%;
    margin-left: 5%;

    .game_chat_message {
      min-height: fit-content;
      width: calc(100% - 12px);
    }
  }

  .game_chat_textbox {
    position: relative;
    height: 5%;
    width: 90%;
    margin-top: 3%;
    margin-left: 5%;
  }
}
</style>
