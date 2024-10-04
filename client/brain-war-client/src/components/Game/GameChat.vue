<template>
  <div class="game_chat">
    <div ref="messages" class="game_chat_messages scrollbar rounded">
      <div class="game_chat_message" :class="{
          mbm: index + 1 !== messages.length,
        }" v-for="(message, index) in messages" :key="index + chatKey">
        <ChatMessage :message="message" />
      </div>
    </div>
    <div class="game_chat_textbox rounded light_background_trans">
      <form ref="chatForm" class="h100" @submit.prevent="submit()">
        <ChatInput @rightIconClicked="rightIconClicked()" />
      </form>
    </div>
  </div>
  <div class="game_chat_textbox rounded light_background_trans">
    
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import ChatMessage from "../Shared/Chat/ChatMessage.vue";
import ChatInput from "../Shared/Inputs/ChatInput.vue";
import IMessage from '@/helpers/interfaces/IMessage';

export default defineComponent({
  name: "GameChat",

  components: { ChatMessage, ChatInput },

  data() {
    return {
      chatKey: 0
    };
  },

  mounted() {
    this.scrollDownToLatestMessages();
    setInterval(() => {
      this.chatKey += 1;
      this.$nextTick(() => {
        this.scrollDownToLatestMessages();
      })
    }, 1000)
  },

  computed: {
    messages(): IMessage[] {
      return this.$store.getters.getMessages
    }
  },

  methods: {
    scrollDownToLatestMessages() {
      const messages = this.$refs.messages as HTMLDivElement;
      if (!messages) {
        return;
      }

      messages.scrollTop = messages.scrollHeight;
    },

    submit() {
      this.$store.dispatch("playAudio", 'click');
      
      if (!this.$store.getters.chatMessageText.trim()) {
        return;
      }


      this.$store.dispatch(
        "sendChatMessage",
        this.$store.getters.chatMessageText,
      );
      this.$store.dispatch("updateChatMessageText", "");
      this.scrollDownToLatestMessages();
    },

    rightIconClicked() {
      this.submit();
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
    width: 92%;
    margin-top: 7%;
    margin-left: 5%;

    .game_chat_message {
      min-height: fit-content;
      width: calc(100% - 12px);
    }
  }

  .game_chat_textbox {
    position: relative;
    height: 7%;
    width: 90%;
    margin-top: 3%;
    margin-left: 5%;
  }
}
</style>
