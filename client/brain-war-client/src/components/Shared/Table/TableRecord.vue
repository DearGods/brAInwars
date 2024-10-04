<template>
  <div
    class="table_record flex"
    :class="{
      disabled: isDisabled && $store.getters.isGameLive,
    }"
  >
    <div
      class="table_cell"
      v-if="record.name"
    >
      <span>
        {{ record.name }}
      </span>
    </div>
    <div
      class="table_cell upppercase"
      v-if="record.action"
    >
      <span>
        {{ record.action }}
      </span>
    </div>
    <div
      class="table_cell upppercase"
      v-if="record.status"
    >
      <span
        :class="{
          light_text: isReady,
          golden_text: isBailed,
        }"
      >
        {{ recordStatus }}
      </span>
    </div>
    <div
      class="table_cell upppercase"
      v-if="record.created_at"
    >
      <span>
        {{ time }}
      </span>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { PLAYER_STATUS_TEXT } from "@/helpers/enums/PlayerStatusEnum";

export default defineComponent({
  name: "TableComponent",
  props: {
    record: {
      type: Object,
      required: true,
    },

    tableCellWidth: {
      type: String,
      required: true,
    },
  },

  data() {
    return {};
  },

  computed: {
    recordStatus(): string {
      if (
        this.record.status.toUpperCase() === PLAYER_STATUS_TEXT.READY &&
        this.$store.getters.isGameLive
      ) {
        return PLAYER_STATUS_TEXT.PLAYING;
      }

      return this.record.status;
    },

    isDisabled(): boolean {
      return (
        this.record.status &&
        this.record.status.toUpperCase() === PLAYER_STATUS_TEXT.WATCHING
      );
    },

    isReady(): boolean {
      return (
        this.record.status &&
        this.record.status.toUpperCase() === PLAYER_STATUS_TEXT.READY
      );
    },

    isBailed(): boolean {
      return (
        this.record.status &&
        this.record.status.toUpperCase() === PLAYER_STATUS_TEXT.BAILED
      );
    },
    time(): string {
      const now = new Date();
      const messageDate = new Date(this.record.created_at);
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

  methods: {},
});
</script>

<style scoped lang="scss">
.table_cell {
  width: 100%;
  text-align: center;
  word-break: break-word;
  line-height: 25px;
  font-size: 0.8em;
}
</style>
