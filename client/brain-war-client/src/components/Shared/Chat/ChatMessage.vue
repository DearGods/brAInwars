<template>
  <div class="game_chat_message rounded spx" :key="chatKey">
    <p class="game_chat_message_name">
      {{ message.name }}
    </p>
    <div class="separator"></div>
    <p class="game_chat_message_content">
      {{ message.message }}
    </p>
    <div class="text_right">
      <small class="game_chat_message_time">
        {{ time }}
      </small>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import IMessage from '@/helpers/interfaces/IMessage';

export default defineComponent({
  name: "GameChat",

  props: {
    message: {
      type: Object as PropType<IMessage>,
      required: true,
    },
  },

  data() {
    return {
      chatKey: 0
    };
  },

  mounted() {
    // setInterval(() => {
    //   this.chatKey += 1;
    //   console.log('cha', this.chatKey);
      
    // }, 1000)
  },

  computed: {
    time(): string {
      const now = new Date();
      const messageDate = new Date(this.message.created_at);
      const seconds = Math.floor((now.getTime() - messageDate.getTime()) / 1000);
      const intervals = {
        year: 31536000,
        month: 2592000,
        week: 604800,
        day: 86400,
        hour: 3600,
        minute: 60,
        second: 1
      };

      for (const [unit, secondsInUnit] of Object.entries(intervals)) {
        const interval = Math.floor(seconds / secondsInUnit);
        if (interval >= 1) {
          return `${interval} ${unit}${interval === 1 ? '' : 's'} ago`;
        }
      }

      return 'NOW';
    }
  },
});
</script>

<style scoped lang="scss">
.game_chat_message {
  padding-top: 4px;
  width: calc(100% - 12px);
  background-color: var(--lightBlueHalfTrans);

  p {
    word-wrap: break-word;
  }

  .game_chat_message_name {
    font-size: 1.1em;
  }

  .game_chat_message_content {
    font-size: 1em;
  }

  .game_chat_message_time {
    font-size: 0.7em;
  }
}
</style>
