<template>
  <div class="table white_text stp">
    <div class="table_header">
      <div class="table_record flex">
        <div
          class="table_cell upppercase"
          v-for="(header, index) in headers"
          :key="index"
        >
          <span>
            {{ header.text }}
          </span>
        </div>
      </div>
    </div>
    <div class="table_body stp">
      <template v-for="(record, index) in records" :key="index">
        <TableRecord :record="record" :tableCellWidth="tableCellWidth" />
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import TableRecord from "./TableRecord.vue";
import IHeader from '@/helpers/interfaces/ITableHeader'

export default defineComponent({
  name: "GameButton",

  components: {
    TableRecord,
  },

  props: {
    headers: {
      type: Array as PropType<IHeader[]>,
      required: true,
    },
    records: {
      type: Array as PropType<any[]>,
      required: true,
    },
    primaryKey: {
      type: String,
      default: "id",
    },
  },

  computed: {
    tableCellWidth(): string {
      return 100 / this.$props.headers.length + "%";
    }
  },

  data() {
    return {};
  },

  methods: {},
});
</script>

<style scoped lang="scss">
.table {
  width: 100%;
  height: 100%;

  .table_body {
    overflow-y: auto;
    max-height: 82%;
  }

  ::-webkit-scrollbar {
    width: 0;
    /* Remove scrollbar width */
    height: 0;
    /* Remove scrollbar height (vertical scrollbar) */
    display: none;
    /* Hide the scrollbar */
  }

  .table_cell {
    width: 100%;
    text-align: center;
  }
}
</style>
