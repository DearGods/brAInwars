<template>
  <div class="game_tabs_wrapper">
    <div class="game_tabs">
      <div class="tab" v-for="(tabText, index) in tabs" :key="index" @click="setActiveTabByIndex(index)"> 
        <h4 :class="{
          'dark_text': index === activeTabIndex,
          'white_text': index !== activeTabIndex,
        }">
          {{ tabText }}
        </h4>
      </div>
    </div>
    <div class="active_tab" :style="`left: ${activeTabLeft}`"></div>
  </div>
</template>

<script lang="ts">
const ACTIVE_TAB_LEFT_SIDE_LEFT = '-125px',
      ACTIVE_TAB_RIGHT_SIDE_LEFT = '165px';
      
import { defineComponent, PropType } from "vue";

export default defineComponent({
  name: "GameTabs",

  props: {
    tabs: {
      type: Array as PropType<Array<string>>,
      required: true,
    },
  },

  data() {
    return {
      activeTabIndex: 0,
      activeTabLeft: ACTIVE_TAB_LEFT_SIDE_LEFT
    };
  },

  computed: {
    
  },

  methods: {
    setActiveTabByIndex(tabIndex: number) {
      this.activeTabIndex = tabIndex; 
      this.activeTabLeft = tabIndex ? ACTIVE_TAB_RIGHT_SIDE_LEFT : ACTIVE_TAB_LEFT_SIDE_LEFT;
      this.$emit('tabPicked', this.tabs[tabIndex]);
    },

    setActiveTab(tabText: string) {
      const tabIndex = this.tabs.findIndex(tab => tab === tabText);
      this.activeTabIndex = tabIndex === -1 ? 0 : tabIndex;
    },
  },
});
</script>

<style scoped lang="scss">
.game_tabs_wrapper {
  width: 100%;
  height: 100%;
  position: relative;
  border-radius: 12px 12px 0 0;
  overflow: hidden;
    
  .game_tabs {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: space-around;
    align-items: center;
  }

  .tab {
    position: relative;
    z-index: 2;
  }

  .active_tab {
    position: absolute;
    top: 2px;
    left: -125px;
    height: 150%;
    width: calc(75% - 50px);
    border-top: 75px solid #bbb;
    border-left: 50px solid transparent;
    border-right: 50px solid transparent;
  }
}
</style>
