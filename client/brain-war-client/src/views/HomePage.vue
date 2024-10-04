<template>
  <div class="home_page align_center content_center flex">
    <img
      :src="imageSrc"
      alt="background"
      class="background_image"
      :class="{
        mobile_background_image: $screen.isSmall,
      }"
    />
    <div class="home_content mp flex align_center">
      <template v-if="rooms.length">
        <div v-if="$screen.isSmall">
          <RoomCard
            v-for="(room, index) in rooms"
            :key="index"
            :room="room"
            :class="{
              stm: index,
            }"
          />
        </div>
        <RoomCard
          v-else
          v-for="(room, index) in rooms"
          :key="index"
          :room="room"
          :class="{
            stm: index,
          }"
        />
      </template>
      <template v-else>
        <h1 class="white_text text_center">
          There are no available games at the moment.
          <br />
          Please try again soon
        </h1>
      </template>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import RoomCard from "../components/Home/RoomCard.vue";
import IRoom from "@/helpers/interfaces/IRoom";
import { DepositModal } from "@/helpers/feTypes";

export default defineComponent({
  name: "HomePage",

  components: {
    RoomCard,
  },

  created() {
    this.$store.dispatch("getGames");
  },

  computed: {
    imageSrc(): string {
      return "https://cdn.brainwars.win/public%2Fimages%2Fhome%2Fbackground.webp";
    },

    rooms(): IRoom[] {
      return this.$store.getters.getRooms;
    },
  },

  methods: {},

  beforeUnmount() {
    this.$store.dispatch("setDepositModalMode", DepositModal.CLOSED);
  },
});
</script>

<style scoped lang="scss">
.home_page {
  height: calc(100vh - 80px);
  width: 100%;
  position: relative;

  .background_image {
    height: calc(100vh - 80px);
    width: 100%;
    object-fit: cover;
    position: absolute;
  }

  .mobile_background_image {
    height: 150%;
  }

  .home_content {
    height: 50vh;
    width: 35vw;
    position: relative;
    top: -10px;
    justify-content: space-between;

    h1 {
      text-transform: uppercase;
    }

    @media only screen and (max-width: 600px) {
      width: 90vw;
      top: -7px;
    }
  }
}
</style>
