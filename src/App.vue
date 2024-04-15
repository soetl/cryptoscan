<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { mdiPencil, mdiPlus } from "@mdi/js";

import CoinCard from "./components/CoinCard.vue";
import AddCoinDialog from "./components/dialogs/AddCoinDialog.vue";
import ChooseCoinsDialog from "./components/dialogs/ChooseCoinsDialog.vue";
import SetTokenDialog from "./components/dialogs/SetTokenDialog.vue";

const theme = ref("light");
const navDrawer = ref(false);
const editMode = ref(false);
const snackbar = ref(false);
const snackbarText = ref("");

const addCoinDialog = ref(false);
const chooseCoinsDialog = ref(false);
const setTokenDialog = ref(false);

const chooseDialogCoins = ref([]);

const coins = ref([]);
coins.value.push({
  id: 1,
  name: "Bitcoin",
  symbol: "BTC",
  price: 0,
  percent_change_24h: 0,
});

findSetting("theme").then((result) => {
  if (result) {
    theme.value = result.value;
  }
});

function toggleTheme() {
  theme.value = theme.value === "light" ? "dark" : "light";
  createSetting("theme", theme.value);
}

async function addCoin(symbol) {
  if (symbol) {
    fetchCoinsBySymbol([symbol]).then((result) => {
      console.log("Fetched coins: ", result);
      if (result.length === 1) {
        createCoins(result);
        addCoinDialog.value = false;
      } else if (result.length > 1) {
        chooseDialogCoins.value = result;
        addCoinDialog.value = false;
        chooseCoinsDialog.value = true;
      }
    });
  }
}

function chooseCoins(coins) {
  if (coins.length > 0) {
    createCoins(coins);
    chooseCoinsDialog.value = false;
  }
}

function removeCoin(id, symbol) {
  if (id) {
    if (deleteCoin(id)) {
      snackbarText.value = `Deleted ${symbol}`;
      snackbar.value = true;
    }
  }
}

function setToken(token) {
  if (token) {
    createSetting("api_key", token).then((result) => {
      if (result) {
        console.log("API key set: ", result);
        snackbarText.value = "API key set";
        snackbar.value = true;
        setTokenDialog.value = false;
      } else {
        snackbarText.value = "Error setting API key";
        snackbar.value = true;
      }
    });
  }
}

function createCoins(coin_list) {
  class CreateCoinsRequest {
    constructor(coins) {
      this.coins = coins;
    }
  }

  coin_list = new CreateCoinsRequest(coin_list);

  invoke("create_coins", { request: coin_list })
    .then((result) => {
      result = JSON.parse(result);
      coins.value = coins.value.filter((i) =>
        result.every((j) => j.id !== i.id)
      );
      coins.value = coins.value.concat(result);
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error(error);
    });
}

function updateCoins(coin_list) {
  const coin_id_list = coin_list.map((coin) => coin.id);
  invoke("update_coins", { request: [coin_id_list] })
    .then((result) => {
      console.log("Updated coins: ", result);
      result = JSON.parse(result);
      snackbarText.value = "Updated coins";
      snackbar.value = true;
      coins.value = coins.value.map((coin) => {
        const updatedCoin = result.find((c) => c.id === coin.id);
        return updatedCoin ? updatedCoin : coin;
      });
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error(error);
    });
}

function deleteCoin(coin_id) {
  class DeleteCoinRequest {
    constructor(id) {
      this.id = id;
    }
  }

  const id = new DeleteCoinRequest(coin_id);

  invoke("delete_coin", { request: id })
    .then((_) => {
      console.log("Deleted coin with id: ", coin_id);
      coins.value = coins.value.filter((coin) => coin.id !== coin_id);
      return true;
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error(error);
      return false;
    });
}

function deleteAllCoins() {
  invoke("delete_all_coins")
    .then((_) => {
      console.log("Deleted all coins");
      snackbarText.value = "Deleted all coins";
      snackbar.value = true;
      coins.value = [];
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error(error);
    });
}

function getAllCoins() {
  invoke("get_all_coins")
    .then((result) => {
      console.log("Got all coins: ", result);
      snackbarText.value = "Got all coins";
      snackbar.value = true;
      coins.value = JSON.parse(result);
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error(error);
    });
}

async function fetchCoinsById(coin_id_list) {
  return invoke("fetch_coins_by_id", { request: [coin_id_list] })
    .then((result) => {
      console.log("Fetched coins by id: ", result);
      return JSON.parse(result);
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error(error);
      return [];
    });
}

async function fetchCoinsBySymbol(coin_symbol_list) {
  console.log("Fetching coins by symbol: ", coin_symbol_list);
  return invoke("fetch_coins_by_symbol", { request: [coin_symbol_list] })
    .then((result) => {
      console.log("Fetched coins sdasadsda: ", result);
      return JSON.parse(result);
    })
    .catch((error) => {
      console.log("Error fetching coins by symbol: ", error);
      snackbarText.value = error;
      snackbar.value = true;
      return [];
    });
}

async function createSetting(key, value) {
  return invoke("create_setting", { request: { key, value } })
    .then((result) => {
      return JSON.parse(result);
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error("Error creating setting: ", error);
      return "";
    });
}

async function findSetting(key) {
  return invoke("find_setting", { request: key })
    .then((result) => {
      return JSON.parse(result);
    })
    .catch((error) => {
      snackbarText.value = error;
      snackbar.value = true;
      console.error("Error finding setting: ", error);
      return "";
    });
}

onMounted(() => {
  getAllCoins();
  
  setInterval(() => {
    updateCoins(coins.value);
  }, 60000);
});
</script>

<template>
  <v-app :theme="theme">
    <v-layout>
      <add-coin-dialog v-model="addCoinDialog" @submit="addCoin" />
      <choose-coins-dialog
        v-model="chooseCoinsDialog"
        :coins="chooseDialogCoins"
        @submit="chooseCoins"
      />
      <set-token-dialog v-model="setTokenDialog" @submit="setToken" />
      <v-app-bar>
        <template v-slot:prepend>
          <v-app-bar-nav-icon @click.stop="navDrawer = !navDrawer" />
        </template>

        <v-app-bar-title text="cryptoscan" />

        <template v-slot:append>
          <v-btn :icon="mdiPencil" @click.stop="editMode = !editMode" />
          <v-btn :icon="mdiPlus" @click.stop="addCoinDialog = !addCoinDialog" />
        </template>
      </v-app-bar>

      <v-navigation-drawer v-model="navDrawer">
        <v-list>
          <v-list-item title="Force update" @click.stop="updateCoins(coins)" />
          <v-list-item
            title="Change API key"
            @click.stop="setTokenDialog = !setTokenDialog"
          />
          <v-list-item title="Clear all coins" @click.stop="deleteAllCoins" />
          <v-list-item title="Toggle theme" @click.stop="toggleTheme" />
        </v-list>
      </v-navigation-drawer>

      <v-main>
        <div class="mt-2">
          <coin-card
            v-for="coin in coins"
            :key="coin.id"
            :coin="coin"
            :editMode="editMode"
            @removeCoin="removeCoin"
          />
        </div>
      </v-main>

      <v-snackbar v-model="snackbar" timeout="2000" :text="snackbarText" />
    </v-layout>
  </v-app>
</template>
