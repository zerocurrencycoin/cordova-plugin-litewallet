package com.litewallet.plugin;

import org.apache.cordova.*;
import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;
import org.apache.commons.io.IOUtils;
import android.util.Log;
import android.content.Context;
import android.util.Base64;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.io.InputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;


import com.rust.litewalletjni.LiteWalletJni;

public class LiteWallet extends CordovaPlugin {

    @Override
    public boolean execute(String action, JSONArray data, CallbackContext callbackContext) throws JSONException {

        Context context = this.cordova.getActivity().getApplicationContext();

        if (action.equals("sync")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("sync", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("syncStatus")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("syncstatus", "")); // Thread-safe.
                }
            });

            return true;


        } else if (action.equals("rescan")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("rescan", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("clear")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("clear", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("info")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("info", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("encryptionstatus")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("encryptionstatus", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("balance")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("balance", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("notes")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("notes", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("privateKey")) {
            final String arg1 = data.getString(0);
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("export", arg1)); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("newZAddress")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("new", "z")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("newTAddress")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("new", "t")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("seed")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("seed", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("send")) {
            final String arg1 = data.getString(0);
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("send", arg1)); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("height")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("height", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("list")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("list", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("encrypt")) {
            final String arg1 = data.getString(0);
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("encrypt", arg1)); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("decrypt")) {
            final String arg1 = data.getString(0);
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("decrypt", arg1)); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("lock")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("lock", "")); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("unlock")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.execute("unlock", "")); // Thread-safe.
                }
            });

            return true;



        } else if (action.equals("save")) {
            final String arg1 = data.getString(0);
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    boolean saved = LiteWalletJni.walletSave(arg1, context);
                    String jsonText = new String();
                    JSONObject obj = new JSONObject();
                    try {
                        obj.put("saved", saved);
                        jsonText = obj.toString();
                    } catch (JSONException e) {
                        e.printStackTrace();
                        callbackContext.success("Error: JSON error!!!");
                    }

                    callbackContext.success(jsonText); // Thread-safe.
                }
            });

            return true;


        } else if (action.equals("getseedphrase")) {
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.getseedphrase()); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("checkseedphrase")) {
            final String arg1 = data.getString(0);
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    callbackContext.success(LiteWalletJni.checkseedphrase(arg1)); // Thread-safe.
                }
            });

            return true;

        } else if (action.equals("exists")) {
            final String arg1 = data.getString(0);
            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    boolean exists = LiteWalletJni.walletExists(arg1, context);
                    String jsonText = new String();
                    JSONObject obj = new JSONObject();
                    try {
                        obj.put("exists", exists);
                        jsonText = obj.toString();
                    } catch (JSONException e) {
                        e.printStackTrace();
                        callbackContext.success("Error: JSON error!!!");
                    };
                    callbackContext.success(jsonText); // Thread-safe.

                }
            });

            return true;

        } else if (action.equals("initalize")) {
            final String arg1 = data.getString(0);
            final String arg2 = data.getString(1);
            final String arg3 = data.getString(2);

            cordova.getThreadPool().execute(new Runnable() {
                public void run() {

                  try {
                      InputStream saplingOutputFile = getClass().getClassLoader().getResourceAsStream("saplingoutput");
                      byte[] saplingOutput = IOUtils.toByteArray(saplingOutputFile);
                      saplingOutputFile.close();

                      InputStream saplingSpendFile = getClass().getClassLoader().getResourceAsStream("saplingspend");
                      byte[] saplingSpend = IOUtils.toByteArray(saplingSpendFile);
                      saplingSpendFile.close();

                      final String arg4 = Base64.encodeToString(saplingOutput, Base64.NO_WRAP);
                      final String arg5 = Base64.encodeToString(saplingSpend, Base64.NO_WRAP);

                      callbackContext.success(LiteWalletJni.initalize(arg1, arg2, arg3, arg4, arg5, context));

                  }
                  catch(IOException e) {
                      e.printStackTrace();
                      callbackContext.error("Error: Wallet initialzation error!!!");
                  }
                }
            });

            return true;

        } else if (action.equals("newWallet")) {
            final String arg1 = data.getString(0);
            final String arg2 = data.getString(1);

            cordova.getThreadPool().execute(new Runnable() {
                public void run() {

                  try {
                      InputStream saplingOutputFile = getClass().getClassLoader().getResourceAsStream("saplingoutput");
                      byte[] saplingOutput = IOUtils.toByteArray(saplingOutputFile);
                      saplingOutputFile.close();

                      InputStream saplingSpendFile = getClass().getClassLoader().getResourceAsStream("saplingspend");
                      byte[] saplingSpend = IOUtils.toByteArray(saplingSpendFile);
                      saplingSpendFile.close();

                      final String arg3 = Base64.encodeToString(saplingOutput, Base64.NO_WRAP);
                      final String arg4 = Base64.encodeToString(saplingSpend, Base64.NO_WRAP);

                      callbackContext.success(LiteWalletJni.walletNew(arg1, arg2, arg3, arg4, context));

                  }
                  catch(IOException e) {
                      e.printStackTrace();
                      callbackContext.error("Error: New wallet error!!!");
                  }
                }
            });

            return true;

        } else if (action.equals("restoreWallet")) {
            final String arg1 = data.getString(0);
            final String arg2 = data.getString(1);
            final String arg3 = data.getString(2);
            final String arg4 = data.getString(3);

            cordova.getThreadPool().execute(new Runnable() {
                public void run() {
                    try {
                      InputStream saplingOutputFile = getClass().getClassLoader().getResourceAsStream("saplingoutput");
                      byte[] saplingOutput = IOUtils.toByteArray(saplingOutputFile);
                      saplingOutputFile.close();

                      InputStream saplingSpendFile = getClass().getClassLoader().getResourceAsStream("saplingspend");
                      byte[] saplingSpend = IOUtils.toByteArray(saplingSpendFile);
                      saplingSpendFile.close();

                      final String arg5 = Base64.encodeToString(saplingOutput, Base64.NO_WRAP);
                      final String arg6 = Base64.encodeToString(saplingSpend, Base64.NO_WRAP);
                      callbackContext.success(LiteWalletJni.walletRestore(arg1, arg2, arg3, arg4, arg5, arg6, context)); // Thread-safe.
                    }
                    catch(IOException e) {
                      e.printStackTrace();
                      callbackContext.error("Error: Restore wallet error!!!");
                    }
                }
            });

            return true;

        } else {
            return false;
        }
    }
}
