<template>
  <div class="game_action_table">
    <Table :headers="headers" :records="records" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Table from "../../components/Shared/Table/Table.vue";

export default defineComponent({
  name: "GameActionTable",

  components: {
    Table,
  },

  props: {
    headers: {
      type: Array,
      required: true,
    },
    records: {
      type: Array,
      required: true,
    },
  },

  data() {
    return {};
  },

  computed: {},

  methods: {},
});
</script>

<style scoped lang="scss">
.game_action_table {
  position: relative;
  width: 100%;
  height: 100%;
}
</style>
