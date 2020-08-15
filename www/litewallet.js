/*global cordova, module*/

module.exports = {
    exists: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "exists", args);
    },

    initalize: function (args, successCallback, errorCallback) {
        cordova.exec(successCallback, errorCallback, "LiteWallet", "initalize", [args]);
    },

    newWallet: function (args, successCallback, errorCallback) {
        cordova.exec(successCallback, errorCallback, "LiteWallet", "newWallet", [args]);
    },

    restoreWallet: function (args, successCallback, errorCallback) {
        cordova.exec(successCallback, errorCallback, "LiteWallet", "restoreWallet", args);
    },

    sync: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "sync", args);
    },

    syncStatus: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "syncStatus", args);
    },

    rescan: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "rescan", args);
    },

    info: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "info", args);
    },

    encryptionstatus: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "encryptionstatus", args);
    },

    balance: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "balance", args);
    },

    notes: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "notes", args);
    },

    privateKey: function (arg, successCallback, errorCallback) {
        cordova.exec(successCallback, errorCallback, "LiteWallet", "privateKey", [args]);
    },

    newZAddress: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "newZAddress", args);
    },

    newTAddress: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "newZAddress", args);
    },

    seed: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "seed", args);
    },

    height: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "height", args);
    },

    list: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "list", args);
    },

    encrypt: function (arg, successCallback, errorCallback) {
        cordova.exec(successCallback, errorCallback, "LiteWallet", "encrypt", [args]);
    },

    decrypt: function (arg, successCallback, errorCallback) {
        cordova.exec(successCallback, errorCallback, "LiteWallet", "decrypt", [args]);
    },

    lock: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "lock", args);
    },

    unlock: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "unlock", args);
    },

    save: function (successCallback, errorCallback) {
      args = []
        cordova.exec(successCallback, errorCallback, "LiteWallet", "save", args);
    },
};
