<template>
  <div class="room_card white_text" @click="joinRoom()">
    <img
      v-if="$screen.isSmall"
      :src="mobileImageSrc"
      alt="mobile room card"
      class="card_image pointer"
    />
    <img
      v-else
      :src="desktopImageSrc"
      alt="desktop room card"
      class="card_image pointer"
    />
    <!-- <div class="room_name">
      <span>
        {{ roomName }}
      </span>
    </div> -->
    <div class="room_bet_price">
      <small>
        <span v-if="!$screen.isSmall">PLAY</span> ${{ room.entry_fee }}
      </small>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import IRoom from "@/helpers/interfaces/IRoom";

export default defineComponent({
  name: "RoomCard",
  props: {
    room: {
      type: Object as PropType<IRoom>,
      required: true,
    },
  },

  data() {
    return {};
  },

  computed: {
    desktopImageSrc(): string {
      return "https://cdn.brainwars.win/public%2Fassets%2Fimages%2Fhome%2Froom_card.webp";
    },

    mobileImageSrc(): string {
      return "https://cdn.brainwars.win/public%2Fassets%2Fimages%2Fhome%2Froom_card_mobile.webp";
    },

    // roomName(): string {
    //   switch (this.room.entry_fee) {
    //     case 10:
    //       return 'BEGINNER'
    //       break;
    //     case 20:
    //       return 'INTERMEDIATE'
    //       break;

    //     default:
    //       return 'BEGINNER'
    //       break;
    //   }
    // }
  },

  methods: {
    joinRoom() {
      this.$store.dispatch("joinRoom", this.room.id);
      this.$store.dispatch("playAudio", "click");
      this.$router.push("/room/" + this.room.id + "?tab=players");
    },
  },
});
</script>

<style scoped lang="scss">
.room_card {
  position: relative;
  transition: 0.2s scale linear;
  padding: 1em;

  &:hover {
    scale: 1.05;
  }

  .card_image {
    box-shadow: 0px 0px 15px 8px #000110;
    outline: 1px solid var(--white);
    border-radius: 25px;
    width: 100%;
    object-fit: contain;

    @media only screen and (max-width: 600px) {
      width: 100%;
      height: auto;
    }
  }

  .room_name {
    top: 8%;
    position: absolute;
    left: 0;
    right: 0;
    margin: auto;
    text-align: center;
    font-size: 1.3em;
    margin-left: 5px;
  }

  .room_bet_price {
    bottom: calc(18% + 2px);
    position: absolute;
    left: 0;
    right: 0;
    margin: auto;
    text-align: center;
    font-size: 1.4em;

    @media only screen and (max-width: 600px) {
      top: 40%;
      font-size: 1.8em;
    }
  }
}
</style>
