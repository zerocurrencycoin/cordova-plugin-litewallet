/*
 * Copyright (C) 2009 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package com.rust.litewalletjni;

import org.apache.cordova.*;
import org.apache.commons.io.IOUtils;
import org.apache.commons.io.FileUtils;
import android.util.Log;
import android.content.Context;
import android.util.Base64;
import java.nio.channels.*;
import java.io.RandomAccessFile;
import java.io.InputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;

public class LiteWalletJni
{

    private static native String initlogging();
    private static native String initnew(final String serveruri, final String saplingOutputb64, final String saplingSpendb64);
    private static native String initfromseed(final String serveruri, final String seed, final String birthday, final String saplingOutputb64, final String saplingSpendb64);
    private static native String initfromb64(final String serveruri, final String datab64, final String saplingOutputb64, final String saplingSpendb64);
    private static native String save();

    public static native String execute(final String cmd, final String args);

    static {
        System.loadLibrary("litewallet-jni");
    }

    public static String initalize(final String serveruri, final String saplingOutputb64, final String saplingSpendb64, final Context context) {
      initlogging();
      String walletSeed = "Error: Wallet file not found!!!";

      File walletFile = new File(context.getFilesDir(),"wallet.dat");
      if (walletFile.exists()) {
        try {
          InputStream walletStream = new FileInputStream(walletFile);
          byte[] wallet = new byte[walletStream.available()];
          walletStream.read(wallet);
          walletStream.close();
          final String datab64 = Base64.encodeToString(wallet, Base64.NO_WRAP);
          walletSeed = initfromb64(serveruri, datab64, saplingOutputb64, saplingSpendb64 );
          }
          catch(IOException e) {
              e.printStackTrace();
              return "Error: Wallet initalization error!!!";
          }
      }
      return walletSeed;
    }

    public static String walletNew(final String serveruri, final String saplingOutputb64, final String saplingSpendb64, final Context context) {
        String walletSeed = "";
        walletSeed = initnew(serveruri, saplingOutputb64, saplingSpendb64); // Thread-safe.
        return walletSeed;
    }

    public static String walletRestore(final String serveruri, final String seed, final String birthday, final String saplingOutputb64, final String saplingSpendb64, final Context context) {
        String walletSeed = "";
        walletSeed = initfromseed(serveruri, seed, birthday, saplingOutputb64, saplingSpendb64); // Thread-safe.
        return walletSeed;
    }


    public static boolean walletSave(Context context) {
        try {

            final String wallet = save();
            File file = new File(context.getFilesDir(),"wallet.dat");
            if (file.exists()) {
                FileChannel channel = new RandomAccessFile(file, "rw").getChannel();
                FileLock lock;

                try {
                    lock = channel.tryLock();
                    FileUtils.writeByteArrayToFile(file, Base64.decode(wallet,Base64.NO_WRAP));
                } catch (OverlappingFileLockException e) {
                    return false;
                }

                if( lock != null ) {
                    lock.release();
                }
                channel.close();
            } else {
                FileUtils.writeByteArrayToFile(file, Base64.decode(wallet,Base64.NO_WRAP));
            }
        }
        catch(IOException e) {
            e.printStackTrace();
            return false;
        }
        return true;
    }

    public static boolean walletExists(Context context) {
        File file = new File(context.getFilesDir(),"wallet.dat");
        return file.exists();
    }


}
