# Requirement Scan

Generated: 2026-02-16 20:34:59Z
TeamTalk symbols discovered: 207

## `TT_AcquireUserAudioBlock`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1774:<li><a class="el" href="group__sounddevices.html#ga48791b35c9b6b0cc27bf2cafbedbc430" title="Extract the raw audio associated with the event CLIENTEVENT_USER_AUDIOBLOCK.">TT_AcquireUserAudioBlock()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2111:<li><a class="el" href="group__sounddevices.html#ga48791b35c9b6b0cc27bf2cafbedbc430" title="Extract the raw audio associated with the event CLIENTEVENT_USER_AUDIOBLOCK.">TT_AcquireUserAudioBlock()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:88:<li>TT_AcquireUserAudioBlock()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:109:<li>TT_AcquireUserAudioBlock()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:632:<p>Call <a class="el" href="group__sounddevices.html#ga48791b35c9b6b0cc27bf2cafbedbc430" title="Extract the raw audio associated with the event CLIENTEVENT_USER_AUDIOBLOCK.">TT_AcquireUserAudioBlock()</a> to extract the <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:122:    [ "TT_AcquireUserAudioBlock", "group__sounddevices.html#ga48791b35c9b6b0cc27bf2cafbedbc430", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:345:<tr class="memitem:ga48791b35c9b6b0cc27bf2cafbedbc430"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="struct_audio_block.html">AudioBlock</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga48791b35c9b6b0cc27bf2cafbedbc430">TT_AcquireUserAudioBlock</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="group__transmission.html#ga6c16695e0994a2ee32d4e93c15daeaaa">StreamTypes</a> uStreamTypes, IN INT32 nUserID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:693:<p>To enable audio blocks first call <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a> then whenever new audio is played the event <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a2615d80d83488d535f761d37cb788a4c" title="A new audio block can be extracted.">CLIENTEVENT_USER_AUDIOBLOCK</a> is generated. Use <a class="el" href="group__sounddevices.html#ga48791b35c9b6b0cc27bf2cafbedbc430" title="Extract the raw audio associated with the event CLIENTEVENT_USER_AUDIOBLOCK.">TT_AcquireUserAudioBlock()</a> to retrieve the audio block.</p>
... (17 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2810:    TEAMTALKDLL_API AudioBlock* TT_AcquireUserAudioBlock(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:428:            let ptr = ffi::api().TT_AcquireUserAudioBlock(self.ptr.0, types, user_id.0);
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_AcquireUserDesktopWindow`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1666:<li><a class="el" href="group__desktopshare.html#ga78350f1bbb5bdf68c9385c2c762344ca" title="Acquire a user&#39;s desktop window (bitmap image).">TT_AcquireUserDesktopWindow()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1670:<li><a class="el" href="group__desktopshare.html#ga20b221fd9754b3d8980e31637421fc13" title="Same as TT_AcquireUserDesktopWindow() except an extra option for converting bitmap to a different for...">TT_AcquireUserDesktopWindowEx()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:594:<p>Use <a class="el" href="group__desktopshare.html#ga78350f1bbb5bdf68c9385c2c762344ca" title="Acquire a user&#39;s desktop window (bitmap image).">TT_AcquireUserDesktopWindow()</a> to retrieve the bitmap of the desktop window.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:91:<li>TT_AcquireUserDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:94:<li>TT_AcquireUserDesktopWindowEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:236:<tr class="memitem:ga78350f1bbb5bdf68c9385c2c762344ca"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="struct_desktop_window.html">DesktopWindow</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga78350f1bbb5bdf68c9385c2c762344ca">TT_AcquireUserDesktopWindow</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:239:<tr class="memitem:ga20b221fd9754b3d8980e31637421fc13"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="struct_desktop_window.html">DesktopWindow</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga20b221fd9754b3d8980e31637421fc13">TT_AcquireUserDesktopWindowEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__desktopshare.html#ga23d1a7c7cf0f6da45ca389904e644d55">BitmapFormat</a> nBitmapFormat)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:240:<tr class="memdesc:ga20b221fd9754b3d8980e31637421fc13"><td class="mdescLeft">&#160;</td><td class="mdescRight">Same as <a class="el" href="group__desktopshare.html#ga78350f1bbb5bdf68c9385c2c762344ca" title="Acquire a user&#39;s desktop window (bitmap image).">TT_AcquireUserDesktopWindow()</a> except an extra option for converting bitmap to a different format.  <a href="group__desktopshare.html#ga20b221fd9754b3d8980e31637421fc13">More...</a><br /></td></tr>
... (31 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2409:    TEAMTALKDLL_API DesktopWindow* TT_AcquireUserDesktopWindow(IN TTInstance* lpTTInstance, 
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2413:    TEAMTALKDLL_API DesktopWindow* TT_AcquireUserDesktopWindowEx(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\desktop.rs:25:            let ptr = ffi::api().TT_AcquireUserDesktopWindow(self.ptr.0, user_id.0);
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_AcquireUserDesktopWindowEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1670:<li><a class="el" href="group__desktopshare.html#ga20b221fd9754b3d8980e31637421fc13" title="Same as TT_AcquireUserDesktopWindow() except an extra option for converting bitmap to a different for...">TT_AcquireUserDesktopWindowEx()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:115:<li>TT_AcquireUserDesktopWindowEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:83:    [ "TT_AcquireUserDesktopWindowEx", "group__desktopshare.html#ga20b221fd9754b3d8980e31637421fc13", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:239:<tr class="memitem:ga20b221fd9754b3d8980e31637421fc13"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="struct_desktop_window.html">DesktopWindow</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga20b221fd9754b3d8980e31637421fc13">TT_AcquireUserDesktopWindowEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__desktopshare.html#ga23d1a7c7cf0f6da45ca389904e644d55">BitmapFormat</a> nBitmapFormat)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1517:<p>To convert bitmap to a different format use <a class="el" href="group__desktopshare.html#ga20b221fd9754b3d8980e31637421fc13" title="Same as TT_AcquireUserDesktopWindow() except an extra option for converting bitmap to a different for...">TT_AcquireUserDesktopWindowEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1531:<h2 class="memtitle"><span class="permalink"><a href="#ga20b221fd9754b3d8980e31637421fc13">&#9670;&nbsp;</a></span>TT_AcquireUserDesktopWindowEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1537:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="struct_desktop_window.html">DesktopWindow</a>* TT_AcquireUserDesktopWindowEx </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:94:<li>TT_AcquireUserDesktopWindowEx()
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2413:    TEAMTALKDLL_API DesktopWindow* TT_AcquireUserDesktopWindowEx(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_AcquireUserMediaVideoFrame`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1658:<li><a class="el" href="group__mediastream.html#gab236763cba33f650ded61d2efe880fe3" title="Extract a user&#39;s media video frame for display.">TT_AcquireUserMediaVideoFrame()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1662:<li><a class="el" href="group__mediastream.html#gaf9a013f71dcd0f0954f2356538cac88a" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserMediaVideoFrame(), so its allocated resou...">TT_ReleaseUserMediaVideoFrame()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:97:<li>TT_AcquireUserMediaVideoFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:118:<li>TT_AcquireUserMediaVideoFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:585:<p>Use <a class="el" href="group__mediastream.html#gab236763cba33f650ded61d2efe880fe3" title="Extract a user&#39;s media video frame for display.">TT_AcquireUserMediaVideoFrame()</a> to display the video frame.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:196:<tr class="memitem:gab236763cba33f650ded61d2efe880fe3"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="struct_video_frame.html">VideoFrame</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#gab236763cba33f650ded61d2efe880fe3">TT_AcquireUserMediaVideoFrame</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:200:<tr class="memdesc:gaf9a013f71dcd0f0954f2356538cac88a"><td class="mdescLeft">&#160;</td><td class="mdescRight">Delete a user's video frame, acquired through <a class="el" href="group__mediastream.html#gab236763cba33f650ded61d2efe880fe3" title="Extract a user&#39;s media video frame for display.">TT_AcquireUserMediaVideoFrame()</a>, so its allocated resources can be released.  <a href="group__mediastream.html#gaf9a013f71dcd0f0954f2356538cac88a">More...</a><br /></td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:208:<p>To stream a media file to a channel call <a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a> and to stop the stream call <a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2" title="Stop streaming media file to channel.">TT_StopStreamingMediaFileToChannel()</a>. The user receiving the media stream can control volume levels by calling <a class="el" href="group__sounddevices.html#gab1826616267c007816091ec4f24d0838" title="Set the volume of a user.">TT_SetUserVolume()</a> and <a class="el" href="group__mediastream.html#gab236763cba33f650ded61d2efe880fe3" title="Extract a user&#39;s media video frame for display.">TT_AcquireUserMediaVideoFrame()</a> to obtain video frames.</p>
... (15 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2279:    TEAMTALKDLL_API VideoFrame* TT_AcquireUserMediaVideoFrame(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:102:            let ptr = ffi::api().TT_AcquireUserMediaVideoFrame(self.ptr.0, user_id.0);
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_AcquireUserVideoCaptureFrame`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1650:<li><a class="el" href="group__videocapture.html#ga21c3d6e6a8cb56b5eef7695e42032990" title="Extract a user&#39;s video capture frame for display.">TT_AcquireUserVideoCaptureFrame()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1654:<li><a class="el" href="group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserVideoCaptureFrame(), so its allocated res...">TT_ReleaseUserVideoCaptureFrame()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1827:<li>Use <a class="el" href="group__videocapture.html#ga21c3d6e6a8cb56b5eef7695e42032990" title="Extract a user&#39;s video capture frame for display.">TT_AcquireUserVideoCaptureFrame()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1831:<li>Use <a class="el" href="group__videocapture.html#ga21c3d6e6a8cb56b5eef7695e42032990" title="Extract a user&#39;s video capture frame for display.">TT_AcquireUserVideoCaptureFrame()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1835:<li>Use <a class="el" href="group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserVideoCaptureFrame(), so its allocated res...">TT_ReleaseUserVideoCaptureFrame()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:100:<li>TT_AcquireUserVideoCaptureFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:121:<li>TT_AcquireUserVideoCaptureFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:576:<p>Use <a class="el" href="group__videocapture.html#ga21c3d6e6a8cb56b5eef7695e42032990" title="Extract a user&#39;s video capture frame for display.">TT_AcquireUserVideoCaptureFrame</a> to display the video frame.</p>
... (23 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2231:    TEAMTALKDLL_API VideoFrame* TT_AcquireUserVideoCaptureFrame(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:69:            let ptr = ffi::api().TT_AcquireUserVideoCaptureFrame(self.ptr.0, user_id.0);
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_AutoPositionUsers`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:103:<li>TT_AutoPositionUsers()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:124:<li>TT_AutoPositionUsers()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:109:    [ "TT_AutoPositionUsers", "group__sounddevices.html#gadd77f73de89bf8ef92335a34933bb265", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:306:<tr class="memitem:gadd77f73de89bf8ef92335a34933bb265"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gadd77f73de89bf8ef92335a34933bb265">TT_AutoPositionUsers</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2077:<h2 class="memtitle"><span class="permalink"><a href="#gadd77f73de89bf8ef92335a34933bb265">&#9670;&nbsp;</a></span>TT_AutoPositionUsers()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2083:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_AutoPositionUsers </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:24:  ['tt_5fautopositionusers_688',['TT_AutoPositionUsers',['../group__sounddevices.html#gadd77f73de89bf8ef92335a34933bb265',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:1371:<div class="line"><a name="l04857"></a><span class="lineno"><a class="line" href="group__sounddevices.html#gadd77f73de89bf8ef92335a34933bb265"> 4857</a></span>&#160;    <a class="code" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="code" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> <a class="code" href="group__sounddevices.html#gadd77f73de89bf8ef92335a34933bb265">TT_AutoPositionUsers</a>(IN <a class="code" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a>* lpTTInstance);</div>
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2115:    TEAMTALKDLL_API TTBOOL TT_AutoPositionUsers(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:267:        unsafe { ffi::api().TT_AutoPositionUsers(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CancelFileTransfer`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:213:<tr class="memitem:ga9d5f435a2a83f1e691f44652f49c80d6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#ga9d5f435a2a83f1e691f44652f49c80d6">TT_CancelFileTransfer</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nTransferID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:1042:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__channels.html#ga9d5f435a2a83f1e691f44652f49c80d6" title="Cancel an active file transfer.">TT_CancelFileTransfer</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:1047:<h2 class="memtitle"><span class="permalink"><a href="#ga9d5f435a2a83f1e691f44652f49c80d6">&#9670;&nbsp;</a></span>TT_CancelFileTransfer()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:1053:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_CancelFileTransfer </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:106:<li>TT_CancelFileTransfer()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:127:<li>TT_CancelFileTransfer()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:83:    [ "TT_CancelFileTransfer", "group__channels.html#ga9d5f435a2a83f1e691f44652f49c80d6", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:25:  ['tt_5fcancelfiletransfer_689',['TT_CancelFileTransfer',['../group__channels.html#ga9d5f435a2a83f1e691f44652f49c80d6',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2826:    TEAMTALKDLL_API TTBOOL TT_CancelFileTransfer(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:88:        unsafe { ffi::api().TT_CancelFileTransfer(self.ptr.0, transfer_id.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CloseDesktopWindow`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2125:<li><a class="el" href="group__desktopshare.html#ga8633bf0e26dc426126f9542696aa4f38" title="Close the current desktop session.">TT_CloseDesktopWindow()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:109:<li>TT_CloseDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:145:<li>TT_CloseDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:194:<tr class="memitem:ga8633bf0e26dc426126f9542696aa4f38"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga8633bf0e26dc426126f9542696aa4f38">TT_CloseDesktopWindow</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:867:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__desktopshare.html#ga8633bf0e26dc426126f9542696aa4f38" title="Close the current desktop session.">TT_CloseDesktopWindow()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:874:<h2 class="memtitle"><span class="permalink"><a href="#ga8633bf0e26dc426126f9542696aa4f38">&#9670;&nbsp;</a></span>TT_CloseDesktopWindow()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:880:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_CloseDesktopWindow </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:68:    [ "TT_CloseDesktopWindow", "group__desktopshare.html#ga8633bf0e26dc426126f9542696aa4f38", null ],
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2295:    TEAMTALKDLL_API TTBOOL TT_CloseDesktopWindow(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\desktop.rs:9:        unsafe { ffi::api().TT_CloseDesktopWindow(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CloseSoundDuplexDevices`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2313:<li><a class="el" href="group__sounddevices.html#gaf0e9524b8222e724a2bfa28f150f6908" title="Shut down sound devices running in duplex mode.">TT_CloseSoundDuplexDevices()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:112:<li>TT_CloseSoundDuplexDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:148:<li>TT_CloseSoundDuplexDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:95:    [ "TT_CloseSoundDuplexDevices", "group__sounddevices.html#gaf0e9524b8222e724a2bfa28f150f6908", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:264:<tr class="memitem:gaf0e9524b8222e724a2bfa28f150f6908"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gaf0e9524b8222e724a2bfa28f150f6908">TT_CloseSoundDuplexDevices</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1046:<p>In order to restart the sound system all sound devices in all client instances must be closed using <a class="el" href="group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e" title="Shutdown the input sound device.">TT_CloseSoundInputDevice()</a>, TT_CloseSoundoutputDevice() and <a class="el" href="group__sounddevices.html#gaf0e9524b8222e724a2bfa28f150f6908" title="Shut down sound devices running in duplex mode.">TT_CloseSoundDuplexDevices()</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1491:<p>Call <a class="el" href="group__sounddevices.html#gaf0e9524b8222e724a2bfa28f150f6908" title="Shut down sound devices running in duplex mode.">TT_CloseSoundDuplexDevices()</a> to shut down duplex mode.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1505:<a class="el" href="group__sounddevices.html#gaf0e9524b8222e724a2bfa28f150f6908" title="Shut down sound devices running in duplex mode.">TT_CloseSoundDuplexDevices()</a> </dd></dl>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2063:    TEAMTALKDLL_API TTBOOL TT_CloseSoundDuplexDevices(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:130:        unsafe { ffi::api().TT_CloseSoundDuplexDevices(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CloseSoundInputDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2482:<li>Call <a class="el" href="group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e" title="Shutdown the input sound device.">TT_CloseSoundInputDevice</a> and <a class="el" href="group__sounddevices.html#ga811ac206ef312d512de58f432d90875d" title="Shutdown the output sound device.">TT_CloseSoundOutputDevice</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2486:<li>Call <a class="el" href="group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e" title="Shutdown the input sound device.">TT_CloseSoundInputDevice</a> and <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice</a> instead. Similar way for output device.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:115:<li>TT_CloseSoundInputDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:151:<li>TT_CloseSoundInputDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:432:   Call #TT_CloseSoundInputDevice and TT_InitSoundInputDevice
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:93:    [ "TT_CloseSoundInputDevice", "group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:258:<tr class="memitem:gaff10e648d33eea6a1561f086db92847e"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e">TT_CloseSoundInputDevice</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1046:<p>In order to restart the sound system all sound devices in all client instances must be closed using <a class="el" href="group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e" title="Shutdown the input sound device.">TT_CloseSoundInputDevice()</a>, TT_CloseSoundoutputDevice() and <a class="el" href="group__sounddevices.html#gaf0e9524b8222e724a2bfa28f150f6908" title="Shut down sound devices running in duplex mode.">TT_CloseSoundDuplexDevices()</a>. </p>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2057:    TEAMTALKDLL_API TTBOOL TT_CloseSoundInputDevice(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:120:        unsafe { ffi::api().TT_CloseSoundInputDevice(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CloseSoundLoopbackTest`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:154:<li>TT_CloseSoundLoopbackTest()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:118:<li>TT_CloseSoundLoopbackTest()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:87:    [ "TT_CloseSoundLoopbackTest", "group__sounddevices.html#gade056c8b74b7b5392ffa8b045f59d515", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:240:<tr class="memitem:gade056c8b74b7b5392ffa8b045f59d515"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gade056c8b74b7b5392ffa8b045f59d515">TT_CloseSoundLoopbackTest</a> (IN <a class="el" href="group__sounddevices.html#ga0b90a2b9785ff1fc52667e5673de800e">TTSoundLoop</a> *lpTTSoundLoop)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1162:<p>Call <a class="el" href="group__sounddevices.html#gade056c8b74b7b5392ffa8b045f59d515" title="Stop recorder and playback test.">TT_CloseSoundLoopbackTest()</a> to stop the loopback test.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1182:<dl class="section return"><dt>Returns</dt><dd>Returns NULL in case of error, otherwise sound loop instance which can be closed by <a class="el" href="group__sounddevices.html#gade056c8b74b7b5392ffa8b045f59d515" title="Stop recorder and playback test.">TT_CloseSoundLoopbackTest()</a>; </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1189:<a class="el" href="group__sounddevices.html#gade056c8b74b7b5392ffa8b045f59d515" title="Stop recorder and playback test.">TT_CloseSoundLoopbackTest()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1194:<h2 class="memtitle"><span class="permalink"><a href="#gade056c8b74b7b5392ffa8b045f59d515">&#9670;&nbsp;</a></span>TT_CloseSoundLoopbackTest()</h2>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2033:    TEAMTALKDLL_API TTBOOL TT_CloseSoundLoopbackTest(IN TTSoundLoop* lpTTSoundLoop);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:491:        unsafe { ffi::api().TT_CloseSoundLoopbackTest(loopback) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CloseSoundOutputDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2482:<li>Call <a class="el" href="group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e" title="Shutdown the input sound device.">TT_CloseSoundInputDevice</a> and <a class="el" href="group__sounddevices.html#ga811ac206ef312d512de58f432d90875d" title="Shutdown the output sound device.">TT_CloseSoundOutputDevice</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:121:<li>TT_CloseSoundOutputDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:157:<li>TT_CloseSoundOutputDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:440:   Call #TT_CloseSoundOutputDevice and TT_InitSoundOutputDevice
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:94:    [ "TT_CloseSoundOutputDevice", "group__sounddevices.html#ga811ac206ef312d512de58f432d90875d", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:261:<tr class="memitem:ga811ac206ef312d512de58f432d90875d"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga811ac206ef312d512de58f432d90875d">TT_CloseSoundOutputDevice</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1397:<a class="el" href="group__sounddevices.html#ga811ac206ef312d512de58f432d90875d" title="Shutdown the output sound device.">TT_CloseSoundOutputDevice</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1540:<h2 class="memtitle"><span class="permalink"><a href="#ga811ac206ef312d512de58f432d90875d">&#9670;&nbsp;</a></span>TT_CloseSoundOutputDevice()</h2>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2060:    TEAMTALKDLL_API TTBOOL TT_CloseSoundOutputDevice(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:125:        unsafe { ffi::api().TT_CloseSoundOutputDevice(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CloseTeamTalk`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:124:<li>TT_CloseTeamTalk()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:160:<li>TT_CloseTeamTalk()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:156:<tr class="memitem:ga93f2c5b3442de8c432e15f0531d057ab"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__initclient.html#ga93f2c5b3442de8c432e15f0531d057ab">TT_CloseTeamTalk</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:227:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__initclient.html#ga93f2c5b3442de8c432e15f0531d057ab" title="Close the TeamTalk client instance and release its resources.">TT_CloseTeamTalk</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:358:<p>This function must be invoked before any other of the TT_* functions can be called. Call <a class="el" href="group__initclient.html#ga93f2c5b3442de8c432e15f0531d057ab" title="Close the TeamTalk client instance and release its resources.">TT_CloseTeamTalk</a> to shutdown the TeamTalk client and release its resources.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:367:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__initclient.html#ga93f2c5b3442de8c432e15f0531d057ab" title="Close the TeamTalk client instance and release its resources.">TT_CloseTeamTalk</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:426:<p>This function must be invoked before any other of the TT_* functions can be called. Call <a class="el" href="group__initclient.html#ga93f2c5b3442de8c432e15f0531d057ab" title="Close the TeamTalk client instance and release its resources.">TT_CloseTeamTalk</a> to shutdown the TeamTalk client and release its resources.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:428:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__initclient.html#ga93f2c5b3442de8c432e15f0531d057ab" title="Close the TeamTalk client instance and release its resources.">TT_CloseTeamTalk</a> </dd></dl>
... (11 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1976:    TEAMTALKDLL_API TTBOOL TT_CloseTeamTalk(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:183:            ffi::api().TT_CloseTeamTalk(ptr);
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_CloseVideoCaptureDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:127:<li>TT_CloseVideoCaptureDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.js:38:    [ "TT_CloseVideoCaptureDevice", "group__videocapture.html#ga08541fdeed23738bbe2f71a8b3fa73f7", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:163:<li>TT_CloseVideoCaptureDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:149:<tr class="memitem:ga08541fdeed23738bbe2f71a8b3fa73f7"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__videocapture.html#ga08541fdeed23738bbe2f71a8b3fa73f7">TT_CloseVideoCaptureDevice</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:385:<a class="el" href="group__videocapture.html#ga08541fdeed23738bbe2f71a8b3fa73f7" title="Close a video capture device.">TT_CloseVideoCaptureDevice</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:390:<h2 class="memtitle"><span class="permalink"><a href="#ga08541fdeed23738bbe2f71a8b3fa73f7">&#9670;&nbsp;</a></span>TT_CloseVideoCaptureDevice()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:396:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_CloseVideoCaptureDevice </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:37:  ['tt_5fclosevideocapturedevice_701',['TT_CloseVideoCaptureDevice',['../group__videocapture.html#ga08541fdeed23738bbe2f71a8b3fa73f7',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2206:    TEAMTALKDLL_API TTBOOL TT_CloseVideoCaptureDevice(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:53:        unsafe { ffi::api().TT_CloseVideoCaptureDevice(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Connect`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:480:<p>The call to <a class="el" href="group__connectivity.html#ga0dd61484f6e2177ff96e3f5027f99861" title="Connect to a server.">TT_Connect()</a> is no longer a blocking call when using encryption.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:615:<p>Both client and server are now able to verify that the remote end is a valid user using TLS peer verification. Peer verfication is set up in <a class="el" href="struct_encryption_context.html" title="Configure peer verification for encrypted connection.">EncryptionContext</a> and enable using <a class="el" href="group__connectivity.html#gae5c3c59f5d71060f68e1266f25bd79e1" title="Setup encryption properties prior to TT_Connect().">TT_SetEncryptionContext()</a> on the client and <a class="el" href="group__serverapi.html#ga7d841aa79e2459e6c66a386d2c09ad80" title="Set certificate and private key for encrypted server.">TTS_SetEncryptionContext()</a> on the server.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:644:<li>New function <a class="el" href="group__connectivity.html#gae5c3c59f5d71060f68e1266f25bd79e1" title="Setup encryption properties prior to TT_Connect().">TT_SetEncryptionContext()</a> for setting up peer verification.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:999:<p>To limit access to your TeamTalk server you can use the <code>szSystemID</code> parameter of TT_StartServerSysID(). When a client has to connect then the chosen system-ID will also have to passed to <a class="el" href="group__connectivity.html#ga0efa81ad43a8491e0c05b0e1cbd3e470" title="Same as TT_Connect() but the option of providing a unique system-ID.">TT_ConnectSysID()</a>. If it's not then the connecting TeamTalk client will receive the <a class="el" href="group__errorhandling.html#ggafc4bdfbf2ff7f70d54e072c3fe3f2c6ca3ca0d6982542919b2b871cf2cbe10533" title="The server uses a protocol which is incompatible with the client instance.">CMDERR_INCOMPATIBLE_PROTOCOLS</a>. The szSystemID of the default TeamTalk application is "teamtalk". This is what you see when you connect with Telnet on port TCP port 10333 (non-encrypted).</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1034:<p>Use the TeamTalk server API to setup a server with a System-ID, i.e. <a class="el" href="group__serverapi.html#ga196df507f6a152b595c637448a0888cc" title="Same as TTS_StartServer() but with the option of specifying a system-ID.">TTS_StartServerSysID()</a> and then use <a class="el" href="group__connectivity.html#ga0efa81ad43a8491e0c05b0e1cbd3e470" title="Same as TT_Connect() but the option of providing a unique system-ID.">TT_ConnectSysID()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1734:<li><a class="el" href="group__connectivity.html#ga0dd61484f6e2177ff96e3f5027f99861" title="Connect to a server.">TT_Connect()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1738:<li><a class="el" href="group__connectivity.html#ga7c5031d5ca33fed6da0622da9f7f70e1" title="Bind to specific IP-address prior to connecting to server.">TT_ConnectEx()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1874:<li><code>TT_ConnectNonEncrypted</code> <ul>
... (72 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2429:    TEAMTALKDLL_API TTBOOL TT_Connect(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2438:    TEAMTALKDLL_API TTBOOL TT_ConnectSysID(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2448:    TEAMTALKDLL_API TTBOOL TT_ConnectEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:317:            ffi::api().TT_Connect(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:339:            ffi::api().TT_ConnectSysID(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:362:            ffi::api().TT_ConnectEx(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\encryption.rs:14:    /// TeamTalk C-API requires this to be configured before `TT_Connect*`.
```

### crates/teamtalk/tests
No matches

### docs
```text
D:\downloads\repos\TeamTalkRust\docs\changelog.md:58:- Manual and auto reconnect paths now enforce a disconnect barrier before retrying `TT_Connect`, matching TeamTalk C-API reconnect requirements.
```

### README.md
No matches

## `TT_ConnectEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1738:<li><a class="el" href="group__connectivity.html#ga7c5031d5ca33fed6da0622da9f7f70e1" title="Bind to specific IP-address prior to connecting to server.">TT_ConnectEx()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2297:<li><a class="el" href="group__connectivity.html#ga7c5031d5ca33fed6da0622da9f7f70e1" title="Bind to specific IP-address prior to connecting to server.">TT_ConnectEx()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:133:<li>TT_ConnectEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:169:<li>TT_ConnectEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:138:<tr class="memitem:ga7c5031d5ca33fed6da0622da9f7f70e1"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__connectivity.html#ga7c5031d5ca33fed6da0622da9f7f70e1">TT_ConnectEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szHostAddress, IN INT32 nTcpPort, IN INT32 nUdpPort, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szBindIPAddr, IN INT32 nLocalTcpPort, IN INT32 nLocalUdpPort, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEncrypted)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:439:<h2 class="memtitle"><span class="permalink"><a href="#ga7c5031d5ca33fed6da0622da9f7f70e1">&#9670;&nbsp;</a></span>TT_ConnectEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:445:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_ConnectEx </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.js:52:    [ "TT_ConnectEx", "group__connectivity.html#ga7c5031d5ca33fed6da0622da9f7f70e1", null ],
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2448:    TEAMTALKDLL_API TTBOOL TT_ConnectEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:362:            ffi::api().TT_ConnectEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_ConnectSysID`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:999:<p>To limit access to your TeamTalk server you can use the <code>szSystemID</code> parameter of TT_StartServerSysID(). When a client has to connect then the chosen system-ID will also have to passed to <a class="el" href="group__connectivity.html#ga0efa81ad43a8491e0c05b0e1cbd3e470" title="Same as TT_Connect() but the option of providing a unique system-ID.">TT_ConnectSysID()</a>. If it's not then the connecting TeamTalk client will receive the <a class="el" href="group__errorhandling.html#ggafc4bdfbf2ff7f70d54e072c3fe3f2c6ca3ca0d6982542919b2b871cf2cbe10533" title="The server uses a protocol which is incompatible with the client instance.">CMDERR_INCOMPATIBLE_PROTOCOLS</a>. The szSystemID of the default TeamTalk application is "teamtalk". This is what you see when you connect with Telnet on port TCP port 10333 (non-encrypted).</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1034:<p>Use the TeamTalk server API to setup a server with a System-ID, i.e. <a class="el" href="group__serverapi.html#ga196df507f6a152b595c637448a0888cc" title="Same as TTS_StartServer() but with the option of specifying a system-ID.">TTS_StartServerSysID()</a> and then use <a class="el" href="group__connectivity.html#ga0efa81ad43a8491e0c05b0e1cbd3e470" title="Same as TT_Connect() but the option of providing a unique system-ID.">TT_ConnectSysID()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:136:<li>TT_ConnectSysID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:172:<li>TT_ConnectSysID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:135:<tr class="memitem:ga0efa81ad43a8491e0c05b0e1cbd3e470"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__connectivity.html#ga0efa81ad43a8491e0c05b0e1cbd3e470">TT_ConnectSysID</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szHostAddress, IN INT32 nTcpPort, IN INT32 nUdpPort, IN INT32 nLocalTcpPort, IN INT32 nLocalUdpPort, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEncrypted, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szSystemID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:357:<h2 class="memtitle"><span class="permalink"><a href="#ga0efa81ad43a8491e0c05b0e1cbd3e470">&#9670;&nbsp;</a></span>TT_ConnectSysID()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:363:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_ConnectSysID </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.js:51:    [ "TT_ConnectSysID", "group__connectivity.html#ga0efa81ad43a8491e0c05b0e1cbd3e470", null ],
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2438:    TEAMTALKDLL_API TTBOOL TT_ConnectSysID(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:339:            ffi::api().TT_ConnectSysID(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DBG_GETDATAPTR`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:139:<li>TT_DBG_GETDATAPTR()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:175:<li>TT_DBG_GETDATAPTR()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:41:  ['tt_5fdbg_5fgetdataptr_705',['TT_DBG_GETDATAPTR',['../_team_talk_8h.html#a84480dcca65c21ca0540371b2d697988',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:1947:<div class="line"><a name="l07880"></a><span class="lineno"><a class="line" href="_team_talk_8h.html#a84480dcca65c21ca0540371b2d697988"> 7880</a></span>&#160;    <a class="code" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> VOID* <a class="code" href="_team_talk_8h.html#a84480dcca65c21ca0540371b2d697988">TT_DBG_GETDATAPTR</a>(IN <a class="code" href="struct_t_t_message.html">TTMessage</a>* pMsg);</div>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:2047:<div class="ttc" id="a_team_talk_8h_html_a84480dcca65c21ca0540371b2d697988"><div class="ttname"><a href="_team_talk_8h.html#a84480dcca65c21ca0540371b2d697988">TT_DBG_GETDATAPTR</a></div><div class="ttdeci">TEAMTALKDLL_API VOID * TT_DBG_GETDATAPTR(IN TTMessage *pMsg)</div></div>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.js:723:    [ "TT_DBG_GETDATAPTR", "_team_talk_8h.html#a84480dcca65c21ca0540371b2d697988", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1695:<tr class="memitem:a84480dcca65c21ca0540371b2d697988"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> VOID *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="_team_talk_8h.html#a84480dcca65c21ca0540371b2d697988">TT_DBG_GETDATAPTR</a> (IN <a class="el" href="struct_t_t_message.html">TTMessage</a> *pMsg)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1936:<h2 class="memtitle"><span class="permalink"><a href="#a84480dcca65c21ca0540371b2d697988">&#9670;&nbsp;</a></span>TT_DBG_GETDATAPTR()</h2>
... (2 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2898:    TEAMTALKDLL_API VOID* TT_DBG_GETDATAPTR(IN TTMessage* pMsg);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:587:        unsafe { ffi::api().TT_DBG_GETDATAPTR(msg) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DBG_SIZEOF`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2361:<li><a class="el" href="_team_talk_8h.html#a9b5397e150222857d566b9f4f7a2841f">TT_DBG_SIZEOF()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:145:<li>TT_DBG_SIZEOF()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:181:<li>TT_DBG_SIZEOF()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:43:  ['tt_5fdbg_5fsizeof_707',['TT_DBG_SIZEOF',['../_team_talk_8h.html#a9b5397e150222857d566b9f4f7a2841f',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\functions_0.js:22:  ['tt_5fdbg_5fsizeof_1174',['TT_DBG_SIZEOF',['../_team_talk_8h.html#a9b5397e150222857d566b9f4f7a2841f',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1693:<tr class="memitem:a9b5397e150222857d566b9f4f7a2841f"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="_team_talk_8h.html#a9b5397e150222857d566b9f4f7a2841f">TT_DBG_SIZEOF</a> (IN <a class="el" href="group__events.html#gaac48a84de49d548073a3f81276a44d57">TTType</a> nType)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1918:<h2 class="memtitle"><span class="permalink"><a href="#a9b5397e150222857d566b9f4f7a2841f">&#9670;&nbsp;</a></span>TT_DBG_SIZEOF()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1924:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DBG_SIZEOF </td>
... (3 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2896:    TEAMTALKDLL_API INT32 TT_DBG_SIZEOF(IN TTType nType);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:582:        unsafe { ffi::api().TT_DBG_SIZEOF(n_type) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DBG_SetSoundInputTone`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:178:<li>TT_DBG_SetSoundInputTone()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:142:<li>TT_DBG_SetSoundInputTone()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:42:  ['tt_5fdbg_5fsetsoundinputtone_706',['TT_DBG_SetSoundInputTone',['../_team_talk_8h.html#a134ac34370054f43d894290e89a5b1e7',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.js:724:    [ "TT_DBG_SetSoundInputTone", "_team_talk_8h.html#a134ac34370054f43d894290e89a5b1e7", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1697:<tr class="memitem:a134ac34370054f43d894290e89a5b1e7"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="_team_talk_8h.html#a134ac34370054f43d894290e89a5b1e7">TT_DBG_SetSoundInputTone</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="group__transmission.html#ga6c16695e0994a2ee32d4e93c15daeaaa">StreamTypes</a> uStreamTypes, IN INT32 nFrequency)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1954:<h2 class="memtitle"><span class="permalink"><a href="#a134ac34370054f43d894290e89a5b1e7">&#9670;&nbsp;</a></span>TT_DBG_SetSoundInputTone()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1960:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_DBG_SetSoundInputTone </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:1949:<div class="line"><a name="l07882"></a><span class="lineno"><a class="line" href="_team_talk_8h.html#a134ac34370054f43d894290e89a5b1e7"> 7882</a></span>&#160;    <a class="code" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="code" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> <a class="code" href="_team_talk_8h.html#a134ac34370054f43d894290e89a5b1e7">TT_DBG_SetSoundInputTone</a>(IN <a class="code" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a>* lpTTInstance,</div>
... (2 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2900:    TEAMTALKDLL_API TTBOOL TT_DBG_SetSoundInputTone(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:563:        unsafe { ffi::api().TT_DBG_SetSoundInputTone(self.ptr.0, stream_types, freq) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DBG_WriteAudioFileTone`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:873:<li>New function <a class="el" href="_team_talk_8h.html#ab8100a432f8888fd64575c2db849fc1b">TT_DBG_WriteAudioFileTone()</a> for generating audio media file.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:184:<li>TT_DBG_WriteAudioFileTone()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:148:<li>TT_DBG_WriteAudioFileTone()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:44:  ['tt_5fdbg_5fwriteaudiofiletone_708',['TT_DBG_WriteAudioFileTone',['../_team_talk_8h.html#ab8100a432f8888fd64575c2db849fc1b',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.js:725:    [ "TT_DBG_WriteAudioFileTone", "_team_talk_8h.html#ab8100a432f8888fd64575c2db849fc1b", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:1953:<div class="line"><a name="l07886"></a><span class="lineno"><a class="line" href="_team_talk_8h.html#ab8100a432f8888fd64575c2db849fc1b"> 7886</a></span>&#160;    <a class="code" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="code" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> <a class="code" href="_team_talk_8h.html#ab8100a432f8888fd64575c2db849fc1b">TT_DBG_WriteAudioFileTone</a>(IN <span class="keyword">const</span> <a class="code" href="struct_media_file_info.html">MediaFileInfo</a>* lpMediaFileInfo,</div>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:2050:<div class="ttc" id="a_team_talk_8h_html_ab8100a432f8888fd64575c2db849fc1b"><div class="ttname"><a href="_team_talk_8h.html#ab8100a432f8888fd64575c2db849fc1b">TT_DBG_WriteAudioFileTone</a></div><div class="ttdeci">TEAMTALKDLL_API TTBOOL TT_DBG_WriteAudioFileTone(IN const MediaFileInfo *lpMediaFileInfo, IN INT32 nFrequency)</div></div>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1699:<tr class="memitem:ab8100a432f8888fd64575c2db849fc1b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="_team_talk_8h.html#ab8100a432f8888fd64575c2db849fc1b">TT_DBG_WriteAudioFileTone</a> (IN const <a class="el" href="struct_media_file_info.html">MediaFileInfo</a> *lpMediaFileInfo, IN INT32 nFrequency)</td></tr>
... (3 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2904:    TEAMTALKDLL_API TTBOOL TT_DBG_WriteAudioFileTone(IN const MediaFileInfo* lpMediaFileInfo,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:576:            ffi::api().TT_DBG_WriteAudioFileTone(&info, freq) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DesktopInput_Execute`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1977:<li><a class="el" href="group__desktopshare.html#ga0461d94c6574dbea6a0fb943deb27115" title="Execute desktop (mouse or keyboard) input.">TT_DesktopInput_Execute</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:151:<li>TT_DesktopInput_Execute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:187:<li>TT_DesktopInput_Execute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:86:    [ "TT_DesktopInput_Execute", "group__desktopshare.html#ga0461d94c6574dbea6a0fb943deb27115", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:248:<tr class="memitem:ga0461d94c6574dbea6a0fb943deb27115"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga0461d94c6574dbea6a0fb943deb27115">TT_DesktopInput_Execute</a> (IN const <a class="el" href="struct_desktop_input.html">DesktopInput</a> *lpDesktopInputs, IN INT32 nDesktopInputCount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:335:<p>In order for a client instance to allow remote desktop access it is required to first subscribe to desktop input from the user who wants access to the shared desktop window. This is done by calling <a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d" title="Subscribe to user events and/or data.">TT_DoSubscribe()</a> along with the user-id and subscription <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443bac180cbf89645f35df10e43eb88012e13" title="Subscribing to STREAMTYPE_DESKTOPINPUT.">SUBSCRIBE_DESKTOPINPUT</a>. Once desktop input (mouse or keyboard input) is received from a remote user the <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a33e0da344e79755aec7abca68640d102" title="Desktop input (mouse or keyboard input) has been received from a user.">CLIENTEVENT_USER_DESKTOPINPUT</a> event will be posted to the client instance. The actual mouse or keyboard input can then be obtained by accessing the <a class="el" href="struct_desktop_input.html" title="A struct containing a mouse or keyboard event.">DesktopInput</a> member of the <a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a>. Afterwards <a class="el" href="group__desktopshare.html#ga0461d94c6574dbea6a0fb943deb27115" title="Execute desktop (mouse or keyboard) input.">TT_DesktopInput_Execute()</a> can be used to execute the mouse or keyboard input.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:486:<p>If <code>uKeyCode</code> in <a class="el" href="struct_desktop_input.html" title="A struct containing a mouse or keyboard event.">DesktopInput</a> is set to <a class="el" href="group__desktopshare.html#gaaae0c5140a5494f88a74ffb36f02f988">TT_DESKTOPINPUT_KEYCODE_IGNORE</a> it means no key (or mouse button) was pressed in the desktop input event and <a class="el" href="group__desktopshare.html#ga0461d94c6574dbea6a0fb943deb27115" title="Execute desktop (mouse or keyboard) input.">TT_DesktopInput_Execute()</a> will ignore the value. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:503:<p>If <code>uMousePosX</code> or <code>uMousePosY</code> in <a class="el" href="struct_desktop_input.html" title="A struct containing a mouse or keyboard event.">DesktopInput</a> are set to <a class="el" href="group__desktopshare.html#ga49d40333bd8bb930660f0303a270636f">TT_DESKTOPINPUT_MOUSEPOS_IGNORE</a> it means the mouse position is ignored when calling <a class="el" href="group__desktopshare.html#ga0461d94c6574dbea6a0fb943deb27115" title="Execute desktop (mouse or keyboard) input.">TT_DesktopInput_Execute()</a>. </p>
... (21 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2858:    TEAMTALKDLL_API INT32 TT_DesktopInput_Execute(IN const DesktopInput* lpDesktopInputs,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DesktopInput_KeyTranslate`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1976:<li><a class="el" href="group__desktopshare.html#ga62c34f3e70d048601e0b6a60cbc930b4" title="Translate platform key-code to and from TeamTalk&#39;s intermediate format.">TT_DesktopInput_KeyTranslate</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:154:<li>TT_DesktopInput_KeyTranslate()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:202:<li>TT_DesktopInput_KeyTranslate()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:245:<tr class="memitem:ga62c34f3e70d048601e0b6a60cbc930b4"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga62c34f3e70d048601e0b6a60cbc930b4">TT_DesktopInput_KeyTranslate</a> (<a class="el" href="group__desktopshare.html#ga71790194c9b03675b34309bb9a526e6f">TTKeyTranslate</a> nTranslate, IN const <a class="el" href="struct_desktop_input.html">DesktopInput</a> *lpDesktopInputs, OUT <a class="el" href="struct_desktop_input.html">DesktopInput</a> *lpTranslatedDesktopInputs, IN INT32 nDesktopInputCount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:342:<p>Because of the issues with keyboard layouts and regional settings the TeamTalk API provides <a class="el" href="group__desktopshare.html#ga62c34f3e70d048601e0b6a60cbc930b4" title="Translate platform key-code to and from TeamTalk&#39;s intermediate format.">TT_DesktopInput_KeyTranslate()</a> which can be used to translate a keyboard's scan-code to an intermediate format. If e.g. a client instance is running Windows then <a class="el" href="group__desktopshare.html#ga62c34f3e70d048601e0b6a60cbc930b4" title="Translate platform key-code to and from TeamTalk&#39;s intermediate format.">TT_DesktopInput_KeyTranslate()</a> can be called with <a class="el" href="group__desktopshare.html#gga71790194c9b03675b34309bb9a526e6fac09a3c2700bd5a4849b9a4a1bc2f296b" title="Translate from Windows scan-code to TTKEYCODE. The Windows scan-code can be retrieved in Windows&#39; WM_...">TTKEY_WINKEYCODE_TO_TTKEYCODE</a> which converts the scan-code on a Windows keyboard to TeamTalk's intermediate format (TTKEYCODE). To be able to execute the key-code once it's received it must be converted back again from TeamTalk's intermediate format to the platform where the application is running. I.e. if the TTKEYCODE is received on a Mac then <a class="el" href="group__desktopshare.html#ga62c34f3e70d048601e0b6a60cbc930b4" title="Translate platform key-code to and from TeamTalk&#39;s intermediate format.">TT_DesktopInput_KeyTranslate()</a> must be called with <a class="el" href="group__desktopshare.html#gga71790194c9b03675b34309bb9a526e6fa56c468ac0664c8851b0b3cacb8355481" title="Translate from TTKEYCODE to Mac OS X Carbon kVK_* key-code.">TTKEY_TTKEYCODE_TO_MACKEYCODE</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1615:<h2 class="memtitle"><span class="permalink"><a href="#ga62c34f3e70d048601e0b6a60cbc930b4">&#9670;&nbsp;</a></span>TT_DesktopInput_KeyTranslate()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1621:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DesktopInput_KeyTranslate </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1708:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__desktopshare.html#ga62c34f3e70d048601e0b6a60cbc930b4" title="Translate platform key-code to and from TeamTalk&#39;s intermediate format.">TT_DesktopInput_KeyTranslate()</a> </dd></dl>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2852:    TEAMTALKDLL_API INT32 TT_DesktopInput_KeyTranslate(TTKeyTranslate nTranslate,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Disconnect`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:157:<li>TT_Disconnect()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:343:<p>This event is posted if <a class="el" href="group__connectivity.html#ga0dd61484f6e2177ff96e3f5027f99861" title="Connect to a server.">TT_Connect</a> fails. Ensure to call <a class="el" href="group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e" title="Disconnect from the server.">TT_Disconnect</a> before calling <a class="el" href="group__connectivity.html#ga0dd61484f6e2177ff96e3f5027f99861" title="Connect to a server.">TT_Connect</a> again.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:353:<p>Ensure to call <a class="el" href="group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e" title="Disconnect from the server.">TT_Disconnect</a> before calling <a class="el" href="group__connectivity.html#ga0dd61484f6e2177ff96e3f5027f99861" title="Connect to a server.">TT_Connect</a> again.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:428:<p>This event is called when a user logs out with <a class="el" href="group__commands.html#gad55b94ebda5761e78687ce68e1cfafe6" title="Logout of the server.">TT_DoLogout</a> or disconnects with <a class="el" href="group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e" title="Disconnect from the server.">TT_Disconnect</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:436:<a class="el" href="group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e" title="Disconnect from the server.">TT_Disconnect</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:211:<li>TT_Disconnect()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:294:<tr><td class="fieldname"><a id="gga58d6e380015b4b1c92c0f09fd6bcfc1ca8934a493a02759c825741f6e4506a41d"></a>CLIENT_CONNECTION&#160;</td><td class="fielddoc"><p>Helper for <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1ca1f0c0ea7738fb72b3027c77fab910b63" title="If set the client instance is currently try to connect to a server, i.e. TT_Connect has been called.">CLIENT_CONNECTING</a> and <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1ca0a688bcbb16a63434cfec434f3c04b84" title="If set the client instance is connected to a server, i.e. CLIENTEVENT_CON_SUCCESS event has been issu...">CLIENT_CONNECTED</a> to see if <a class="el" href="group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e" title="Disconnect from the server.">TT_Disconnect</a> should be called. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.js:53:    [ "TT_Disconnect", "group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e", null ],
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2458:    TEAMTALKDLL_API TTBOOL TT_Disconnect(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:376:        unsafe { ffi::api().TT_Disconnect(ptr) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoBan`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:935:<li>New function <a class="el" href="group__commands.html#ga59eaf48b600f4053974028f39e7d911e" title="Ban the user with nUserID using the ban types specified.">TT_DoBanUserEx()</a> for banning a user using <a class="el" href="group__server.html#ga1599cea2092c823d60882c5d9318e538" title="Way to ban a user from either login or joining a channel.">BanType</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:936:<li>New function <a class="el" href="group__commands.html#ga956dd0a7d3d0fd5174275b6fd14ce981" title="Ban the properties specified in lpBannedUser.">TT_DoBan()</a> for banning properties, i.e. <a class="el" href="struct_banned_user.html" title="A struct containing the properties of a banned user.">BannedUser</a>.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1993:<p>Previously it was only possible to ban a user's IP-address if the user was present on the server. Now it's, however, possible to ban an IP-address using <a class="el" href="group__commands.html#ga957564773fbaa48478ff0c6b97e937ba" title="Issue a ban command on an IP-address user.">TT_DoBanIPAddress()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2020:<li><a class="el" href="group__commands.html#ga957564773fbaa48478ff0c6b97e937ba" title="Issue a ban command on an IP-address user.">TT_DoBanIPAddress()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:160:<li>TT_DoBan()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:163:<li>TT_DoBanIPAddress()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:166:<li>TT_DoBanUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:169:<li>TT_DoBanUserEx()
... (67 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2604:    TEAMTALKDLL_API INT32 TT_DoBanUser(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2609:    TEAMTALKDLL_API INT32 TT_DoBanUserEx(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2614:    TEAMTALKDLL_API INT32 TT_DoBan(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2618:    TEAMTALKDLL_API INT32 TT_DoBanIPAddress(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:46:        unsafe { ffi::api().TT_DoBanIPAddress(self.ptr.0, ip.tt().as_ptr(), ban_type) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:329:        unsafe { ffi::api().TT_DoBanUser(self.ptr.0, user_id.0, channel_id.0) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:337:        unsafe { ffi::api().TT_DoBanUserEx(self.ptr.0, user_id.0, ban_types) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:418:        unsafe { ffi::api().TT_DoBan(self.ptr.0, &banned_user.to_ffi()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoBanIPAddress`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1993:<p>Previously it was only possible to ban a user's IP-address if the user was present on the server. Now it's, however, possible to ban an IP-address using <a class="el" href="group__commands.html#ga957564773fbaa48478ff0c6b97e937ba" title="Issue a ban command on an IP-address user.">TT_DoBanIPAddress()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2020:<li><a class="el" href="group__commands.html#ga957564773fbaa48478ff0c6b97e937ba" title="Issue a ban command on an IP-address user.">TT_DoBanIPAddress()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:163:<li>TT_DoBanIPAddress()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:217:<li>TT_DoBanIPAddress()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:184:<tr class="memitem:ga957564773fbaa48478ff0c6b97e937ba"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga957564773fbaa48478ff0c6b97e937ba">TT_DoBanIPAddress</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szIPAddress, IN INT32 nChannelID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1645:<a class="el" href="group__commands.html#ga957564773fbaa48478ff0c6b97e937ba" title="Issue a ban command on an IP-address user.">TT_DoBanIPAddress()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1734:<h2 class="memtitle"><span class="permalink"><a href="#ga957564773fbaa48478ff0c6b97e937ba">&#9670;&nbsp;</a></span>TT_DoBanIPAddress()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1740:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoBanIPAddress </td>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2618:    TEAMTALKDLL_API INT32 TT_DoBanIPAddress(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:46:        unsafe { ffi::api().TT_DoBanIPAddress(self.ptr.0, ip.tt().as_ptr(), ban_type) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoBanUser`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:935:<li>New function <a class="el" href="group__commands.html#ga59eaf48b600f4053974028f39e7d911e" title="Ban the user with nUserID using the ban types specified.">TT_DoBanUserEx()</a> for banning a user using <a class="el" href="group__server.html#ga1599cea2092c823d60882c5d9318e538" title="Way to ban a user from either login or joining a channel.">BanType</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:220:<li>TT_DoBanUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:223:<li>TT_DoBanUserEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:29:    [ "TT_DoBanUser", "group__commands.html#gafc60932451858a91a7c79887183f5707", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:30:    [ "TT_DoBanUserEx", "group__commands.html#ga59eaf48b600f4053974028f39e7d911e", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:175:<tr class="memitem:gafc60932451858a91a7c79887183f5707"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gafc60932451858a91a7c79887183f5707">TT_DoBanUser</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN INT32 nChannelID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:178:<tr class="memitem:ga59eaf48b600f4053974028f39e7d911e"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga59eaf48b600f4053974028f39e7d911e">TT_DoBanUserEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__server.html#gaa0c2675bd79ed07d050991184d53d3b0">BanTypes</a> uBanTypes)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:863:<p>To ban a user call <a class="el" href="group__commands.html#gafc60932451858a91a7c79887183f5707" title="Issue a ban command on a user.">TT_DoBanUser</a> before <a class="el" href="group__commands.html#gace2bd2cc8f703d9dad0ffe43a43e7dba" title="Kick user from either channel or server.">TT_DoKickUser</a>.</p>
... (34 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2604:    TEAMTALKDLL_API INT32 TT_DoBanUser(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2609:    TEAMTALKDLL_API INT32 TT_DoBanUserEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:329:        unsafe { ffi::api().TT_DoBanUser(self.ptr.0, user_id.0, channel_id.0) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:337:        unsafe { ffi::api().TT_DoBanUserEx(self.ptr.0, user_id.0, ban_types) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoBanUserEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:935:<li>New function <a class="el" href="group__commands.html#ga59eaf48b600f4053974028f39e7d911e" title="Ban the user with nUserID using the ban types specified.">TT_DoBanUserEx()</a> for banning a user using <a class="el" href="group__server.html#ga1599cea2092c823d60882c5d9318e538" title="Way to ban a user from either login or joining a channel.">BanType</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:169:<li>TT_DoBanUserEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:223:<li>TT_DoBanUserEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:30:    [ "TT_DoBanUserEx", "group__commands.html#ga59eaf48b600f4053974028f39e7d911e", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:178:<tr class="memitem:ga59eaf48b600f4053974028f39e7d911e"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga59eaf48b600f4053974028f39e7d911e">TT_DoBanUserEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__server.html#gaa0c2675bd79ed07d050991184d53d3b0">BanTypes</a> uBanTypes)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1651:<a class="el" href="group__commands.html#ga59eaf48b600f4053974028f39e7d911e" title="Ban the user with nUserID using the ban types specified.">TT_DoBanUserEx()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1656:<h2 class="memtitle"><span class="permalink"><a href="#ga59eaf48b600f4053974028f39e7d911e">&#9670;&nbsp;</a></span>TT_DoBanUserEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1662:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoBanUserEx </td>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2609:    TEAMTALKDLL_API INT32 TT_DoBanUserEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:337:        unsafe { ffi::api().TT_DoBanUserEx(self.ptr.0, user_id.0, ban_types) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoChangeNickname`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:217:<li><a class="el" href="changelog.html#v52csrvcb">Server callbacks for TT_DoChangeNickname() and TT_DoChangeStatus()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:923:<li>Added <a class="el" href="group__server.html#ga5665cc8959eb306b85b6ac008cc043e7" title="The rights users have once they have logged on to the server.">UserRight</a> to lock nickname: <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679a53e1ab8202e08c725787ee91ac847945" title="User&#39;s nick name is locked. TT_DoChangeNickname() cannot be used and TT_DoLogin() will ignore szNickn...">USERRIGHT_LOCKED_NICKNAME</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:960:Server callbacks for TT_DoChangeNickname() and TT_DoChangeStatus()</h3>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2550:<li>Call <a class="el" href="group__commands.html#gaf964f4cad5921cbb2db8b8f3938cc536" title="Change the client instance&#39;s nick name.">TT_DoChangeNickname</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:172:<li>TT_DoChangeNickname()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:226:<li>TT_DoChangeNickname()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:289:   @see TT_DoChangeNickname
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:10:    [ "TT_DoChangeNickname", "group__commands.html#gaf964f4cad5921cbb2db8b8f3938cc536", null ],
... (19 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2513:    TEAMTALKDLL_API INT32 TT_DoChangeNickname(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:287:        unsafe { ffi::api().TT_DoChangeNickname(self.ptr.0, nick.tt().as_ptr()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoChangeStatus`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:217:<li><a class="el" href="changelog.html#v52csrvcb">Server callbacks for TT_DoChangeNickname() and TT_DoChangeStatus()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:924:<li>Added <a class="el" href="group__server.html#ga5665cc8959eb306b85b6ac008cc043e7" title="The rights users have once they have logged on to the server.">UserRight</a> to lock status: <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679ad2e058c9ed93e9277a3d6606bec846f5" title="User&#39;s status is locked. TT_DoChangeStatus() cannot be used.">USERRIGHT_LOCKED_STATUS</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:960:Server callbacks for TT_DoChangeNickname() and TT_DoChangeStatus()</h3>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:175:<li>TT_DoChangeStatus()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:229:<li>TT_DoChangeStatus()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:11:    [ "TT_DoChangeStatus", "group__commands.html#gaec0cb298035486f8c350bfe22123ac30", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:121:<tr class="memitem:gaec0cb298035486f8c350bfe22123ac30"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gaec0cb298035486f8c350bfe22123ac30">TT_DoChangeStatus</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nStatusMode, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szStatusMessage)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:604:<h2 class="memtitle"><span class="permalink"><a href="#gaec0cb298035486f8c350bfe22123ac30">&#9670;&nbsp;</a></span>TT_DoChangeStatus()</h2>
... (21 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2517:    TEAMTALKDLL_API INT32 TT_DoChangeStatus(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:284:        unsafe { ffi::api().TT_DoChangeStatus(ptr, status_mode, message.tt().as_ptr()) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:296:            ffi::api().TT_DoChangeStatus(self.ptr.0, status.to_bits() as i32, msg.tt().as_ptr())
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoChannelOp`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2349:<li><a class="el" href="group__commands.html#ga4f454f3b75cdc179693e0c0e39528132" title="Make another user operator of a channel using the szOpPassword of Channel.">TT_DoChannelOpEx()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:13:    [ "TT_DoChannelOp", "group__commands.html#ga652d9fa6077d6c19b8bfd0e82961b563", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:14:    [ "TT_DoChannelOpEx", "group__commands.html#ga4f454f3b75cdc179693e0c0e39528132", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:260:<p>Sometimes it may be necessary to kick and ban users from a server. With <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679a7b846bbbd3909e9d62f770b46a6854d0" title="User can kick users off the server.">USERRIGHT_KICK_USERS</a> it is possible to use the command <a class="el" href="group__commands.html#gace2bd2cc8f703d9dad0ffe43a43e7dba" title="Kick user from either channel or server.">TT_DoKickUser()</a> to kick a user off the server. A channel operator (<a class="el" href="group__commands.html#ga652d9fa6077d6c19b8bfd0e82961b563" title="Make another user operator of a channel.">TT_DoChannelOp()</a>) can also kick a user from a channel (but not off a server).</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:522:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#ga652d9fa6077d6c19b8bfd0e82961b563" title="Make another user operator of a channel.">TT_DoChannelOp()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:127:<tr class="memitem:ga652d9fa6077d6c19b8bfd0e82961b563"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga652d9fa6077d6c19b8bfd0e82961b563">TT_DoChannelOp</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN INT32 nChannelID, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bMakeOperator)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:130:<tr class="memitem:ga4f454f3b75cdc179693e0c0e39528132"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga4f454f3b75cdc179693e0c0e39528132">TT_DoChannelOpEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN INT32 nChannelID, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szOpPassword, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bMakeOperator)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:702:<h2 class="memtitle"><span class="permalink"><a href="#ga652d9fa6077d6c19b8bfd0e82961b563">&#9670;&nbsp;</a></span>TT_DoChannelOp()</h2>
... (31 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2526:    TEAMTALKDLL_API INT32 TT_DoChannelOp(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2532:    TEAMTALKDLL_API INT32 TT_DoChannelOpEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:528:            ffi::api().TT_DoChannelOpEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoChannelOpEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2349:<li><a class="el" href="group__commands.html#ga4f454f3b75cdc179693e0c0e39528132" title="Make another user operator of a channel using the szOpPassword of Channel.">TT_DoChannelOpEx()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:181:<li>TT_DoChannelOpEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:235:<li>TT_DoChannelOpEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:14:    [ "TT_DoChannelOpEx", "group__commands.html#ga4f454f3b75cdc179693e0c0e39528132", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:130:<tr class="memitem:ga4f454f3b75cdc179693e0c0e39528132"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga4f454f3b75cdc179693e0c0e39528132">TT_DoChannelOpEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN INT32 nChannelID, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szOpPassword, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bMakeOperator)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:759:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#ga4f454f3b75cdc179693e0c0e39528132" title="Make another user operator of a channel using the szOpPassword of Channel.">TT_DoChannelOpEx</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:764:<h2 class="memtitle"><span class="permalink"><a href="#ga4f454f3b75cdc179693e0c0e39528132">&#9670;&nbsp;</a></span>TT_DoChannelOpEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:770:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoChannelOpEx </td>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2532:    TEAMTALKDLL_API INT32 TT_DoChannelOpEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:528:            ffi::api().TT_DoChannelOpEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoDeleteFile`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:184:<li>TT_DoDeleteFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:238:<li>TT_DoDeleteFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:18:    [ "TT_DoDeleteFile", "group__commands.html#ga66712aea48ec45e3433cdc9059737dab", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:142:<tr class="memitem:ga66712aea48ec45e3433cdc9059737dab"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga66712aea48ec45e3433cdc9059737dab">TT_DoDeleteFile</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID, IN INT32 nFileID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1019:<h2 class="memtitle"><span class="permalink"><a href="#ga66712aea48ec45e3433cdc9059737dab">&#9670;&nbsp;</a></span>TT_DoDeleteFile()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1025:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoDeleteFile </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:281:   @see TT_DoDeleteFile
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:369:   @see TT_DoDeleteFile
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2555:    TEAMTALKDLL_API INT32 TT_DoDeleteFile(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:70:        unsafe { ffi::api().TT_DoDeleteFile(self.ptr.0, channel_id.0, remote_file_id.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoDeleteUserAccount`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2431:<li>Call <a class="el" href="group__commands.html#gaaaee2023a14c21a59dba48eaff2e9b3c" title="Issue command to delete a user account on the server.">TT_DoDeleteUserAccount</a> to delete a user account.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:187:<li>TT_DoDeleteUserAccount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:241:<li>TT_DoDeleteUserAccount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:172:<tr class="memitem:gaaaee2023a14c21a59dba48eaff2e9b3c"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gaaaee2023a14c21a59dba48eaff2e9b3c">TT_DoDeleteUserAccount</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szUsername)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1531:<a class="el" href="group__commands.html#gaaaee2023a14c21a59dba48eaff2e9b3c" title="Issue command to delete a user account on the server.">TT_DoDeleteUserAccount</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1540:<h2 class="memtitle"><span class="permalink"><a href="#gaaaee2023a14c21a59dba48eaff2e9b3c">&#9670;&nbsp;</a></span>TT_DoDeleteUserAccount()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1546:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoDeleteUserAccount </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:28:    [ "TT_DoDeleteUserAccount", "group__commands.html#gaaaee2023a14c21a59dba48eaff2e9b3c", null ],
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2600:    TEAMTALKDLL_API INT32 TT_DoDeleteUserAccount(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:482:        unsafe { ffi::api().TT_DoDeleteUserAccount(self.ptr.0, username.tt().as_ptr()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoJoinChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:612:<p>Use <a class="el" href="group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c" title="Make a new channel on the server.">TT_DoMakeChannel()</a> or <a class="el" href="group__commands.html#ga8b15a791c8034ec640a4a03435704333" title="Create a new channel and join it.">TT_DoJoinChannel()</a> to create a hidden channel.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2342:<li>When creating a <a class="el" href="struct_channel.html" title="A struct containing the properties of a channel.">Channel</a> using <a class="el" href="group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c" title="Make a new channel on the server.">TT_DoMakeChannel()</a> or <a class="el" href="group__commands.html#ga8b15a791c8034ec640a4a03435704333" title="Create a new channel and join it.">TT_DoJoinChannel()</a> it is now possible to specify an audio configuration so all users will speak at the same volume level.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:190:<li>TT_DoJoinChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:193:<li>TT_DoJoinChannelByID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:230:<p>With <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679af5c65ca97729d4eea932ba8755bfc454" title="User is allowed to create temporary channels which disappear when last user leaves the channel.">USERRIGHT_CREATE_TEMPORARY_CHANNEL</a> the user can only create temporary channels which disappear when the last user leaves the channel. A temporary channel must be created by calling <a class="el" href="group__commands.html#ga8b15a791c8034ec640a4a03435704333" title="Create a new channel and join it.">TT_DoJoinChannel()</a>. Once a user has created a temporary channel and joined it he becomes operator of the channel which means that he can update the channel's properties at any given time.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:244:<li>TT_DoJoinChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:247:<li>TT_DoJoinChannelByID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:252:<a class="el" href="group__commands.html#ga8b15a791c8034ec640a4a03435704333" title="Create a new channel and join it.">TT_DoJoinChannel</a> </dd></dl>
... (49 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2501:    TEAMTALKDLL_API INT32 TT_DoJoinChannel(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2505:    TEAMTALKDLL_API INT32 TT_DoJoinChannelByID(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:272:        unsafe { ffi::api().TT_DoJoinChannelByID(ptr, channel_id, password.tt().as_ptr()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoJoinChannelByID`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:247:<li>TT_DoJoinChannelByID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:8:    [ "TT_DoJoinChannelByID", "group__commands.html#ga11c00e7d740f94a8c628e9f3a85ee97c", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:193:<li>TT_DoJoinChannelByID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:258:<pre class="fragment">   The #TT_DoJoinChannel or #TT_DoJoinChannelByID passed an
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:271:<p><a class="el" href="group__commands.html#ga8b15a791c8034ec640a4a03435704333" title="Create a new channel and join it.">TT_DoJoinChannel</a> or <a class="el" href="group__commands.html#ga11c00e7d740f94a8c628e9f3a85ee97c" title="Join an existing channel.">TT_DoJoinChannelByID</a> failed because no more users are allowed in the channel. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:283:   @see TT_DoJoinChannelByID
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:337:   @see TT_DoJoinChannelByID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:356:<pre class="fragment">   #TT_DoJoinChannel or #TT_DoJoinChannelByID failed because
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2505:    TEAMTALKDLL_API INT32 TT_DoJoinChannelByID(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:272:        unsafe { ffi::api().TT_DoJoinChannelByID(ptr, channel_id, password.tt().as_ptr()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoKickUser`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:250:<li>TT_DoKickUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:196:<li>TT_DoKickUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:133:<tr class="memitem:gace2bd2cc8f703d9dad0ffe43a43e7dba"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gace2bd2cc8f703d9dad0ffe43a43e7dba">TT_DoKickUser</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN INT32 nChannelID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:831:<h2 class="memtitle"><span class="permalink"><a href="#gace2bd2cc8f703d9dad0ffe43a43e7dba">&#9670;&nbsp;</a></span>TT_DoKickUser()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:837:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoKickUser </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:863:<p>To ban a user call <a class="el" href="group__commands.html#gafc60932451858a91a7c79887183f5707" title="Issue a ban command on a user.">TT_DoBanUser</a> before <a class="el" href="group__commands.html#gace2bd2cc8f703d9dad0ffe43a43e7dba" title="Kick user from either channel or server.">TT_DoKickUser</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1623:<p>The ban applies to the user's IP-address. Call <a class="el" href="group__commands.html#gace2bd2cc8f703d9dad0ffe43a43e7dba" title="Kick user from either channel or server.">TT_DoKickUser</a> to kick the user off the server.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1641:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gace2bd2cc8f703d9dad0ffe43a43e7dba" title="Kick user from either channel or server.">TT_DoKickUser()</a> </dd>
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2539:    TEAMTALKDLL_API INT32 TT_DoKickUser(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:321:        unsafe { ffi::api().TT_DoKickUser(self.ptr.0, user_id.0, channel_id.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoLeaveChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:199:<li>TT_DoLeaveChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:253:<li>TT_DoLeaveChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:9:    [ "TT_DoLeaveChannel", "group__commands.html#gad97fb9556f7afc29cf9290c7cdcdad30", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:115:<tr class="memitem:gad97fb9556f7afc29cf9290c7cdcdad30"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gad97fb9556f7afc29cf9290c7cdcdad30">TT_DoLeaveChannel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:452:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gad97fb9556f7afc29cf9290c7cdcdad30" title="Leave the current channel.">TT_DoLeaveChannel</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:512:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gad97fb9556f7afc29cf9290c7cdcdad30" title="Leave the current channel.">TT_DoLeaveChannel</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:523:<h2 class="memtitle"><span class="permalink"><a href="#gad97fb9556f7afc29cf9290c7cdcdad30">&#9670;&nbsp;</a></span>TT_DoLeaveChannel()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:529:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoLeaveChannel </td>
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2510:    TEAMTALKDLL_API INT32 TT_DoLeaveChannel(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:276:        unsafe { ffi::api().TT_DoLeaveChannel(ptr) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoListBans`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:256:<li>TT_DoListBans()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:202:<li>TT_DoListBans()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:35:    [ "TT_DoListBans", "group__commands.html#ga5914cefb1b0e58432e0484373098a2e6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:193:<tr class="memitem:ga5914cefb1b0e58432e0484373098a2e6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga5914cefb1b0e58432e0484373098a2e6">TT_DoListBans</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID, IN INT32 nIndex, IN INT32 nCount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1643:<a class="el" href="group__commands.html#ga5914cefb1b0e58432e0484373098a2e6" title="Issue a command to list the banned users.">TT_DoListBans()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1692:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#ga5914cefb1b0e58432e0484373098a2e6" title="Issue a command to list the banned users.">TT_DoListBans()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1727:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#ga5914cefb1b0e58432e0484373098a2e6" title="Issue a command to list the banned users.">TT_DoListBans()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1785:<a class="el" href="group__commands.html#ga5914cefb1b0e58432e0484373098a2e6" title="Issue a command to list the banned users.">TT_DoListBans</a> </dd></dl>
... (15 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2631:    TEAMTALKDLL_API INT32 TT_DoListBans(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:64:        unsafe { ffi::api().TT_DoListBans(self.ptr.0, channel_id.0, index, count) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoListUserAccounts`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2432:<li>Call <a class="el" href="group__commands.html#gad706c94a8343b377ddf2d8bf63eb8c54" title="Issue command to list user accounts on the server.">TT_DoListUserAccounts</a> to list all user accounts on the server.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:205:<li>TT_DoListUserAccounts()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:259:<li>TT_DoListUserAccounts()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:304:   @see TT_DoListUserAccounts
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:26:    [ "TT_DoListUserAccounts", "group__commands.html#gad706c94a8343b377ddf2d8bf63eb8c54", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:166:<tr class="memitem:gad706c94a8343b377ddf2d8bf63eb8c54"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gad706c94a8343b377ddf2d8bf63eb8c54">TT_DoListUserAccounts</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nIndex, IN INT32 nCount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1432:<h2 class="memtitle"><span class="permalink"><a href="#gad706c94a8343b377ddf2d8bf63eb8c54">&#9670;&nbsp;</a></span>TT_DoListUserAccounts()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1438:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoListUserAccounts </td>
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2591:    TEAMTALKDLL_API INT32 TT_DoListUserAccounts(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:466:        unsafe { ffi::api().TT_DoListUserAccounts(self.ptr.0, index, count) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoLogin`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:923:<li>Added <a class="el" href="group__server.html#ga5665cc8959eb306b85b6ac008cc043e7" title="The rights users have once they have logged on to the server.">UserRight</a> to lock nickname: <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679a53e1ab8202e08c725787ee91ac847945" title="User&#39;s nick name is locked. TT_DoChangeNickname() cannot be used and TT_DoLogin() will ignore szNickn...">USERRIGHT_LOCKED_NICKNAME</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1000:<p>On top of the system-ID restriction it's now also possible to do a check on the client name connecting by using the extended <a class="el" href="group__commands.html#ga3f195e405b5598d86663975dce37e401" title="Logon to a server.">TT_DoLoginEx()</a>. When a client tries to connect to a TeamTalk server you can then check the <code>szClientName</code> property of <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a> to ensure only your client application is allowed to connect.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1030:<p>Added <code>szClientName</code> to <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a>-struct. The client name is specified in <a class="el" href="group__commands.html#ga3f195e405b5598d86663975dce37e401" title="Logon to a server.">TT_DoLoginEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1742:<li><a class="el" href="group__commands.html#ga9d15454938054ddb66ebe16f88e2efaa" title="Same as TT_DologinEx() but without the option to specify szClientName. Kept for backwards compatibili...">TT_DoLogin()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2382:<p>After login it's now possible to retrieve one's own <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> by calling <a class="el" href="group__users.html#ga9ae176938d27d34bd719dc3df89407d7" title="Get the local client instance&#39;s UserAccount.">TT_GetMyUserAccount()</a>. <a class="el" href="group__users.html#gae8dd6c7fbdeead08735e9ce83e16d1b2" title="If an account was used in TT_DoLogin then this value will return the nUserData from the UserAccount.">TT_GetMyUserData()</a> can be used to extract one's <em>nUserData</em> of one's <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:208:<li>TT_DoLogin()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:211:<li>TT_DoLoginEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:160:<p>To connect to a server the user application must call <a class="el" href="group__connectivity.html#ga0dd61484f6e2177ff96e3f5027f99861" title="Connect to a server.">TT_Connect</a>. Once connected the event <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a7232b1ffe4392a480b0b86069e2fb2f3" title="Connected successfully to the server.">CLIENTEVENT_CON_SUCCESS</a> is posted to the user application and the <a class="el" href="group__commands.html#ga9d15454938054ddb66ebe16f88e2efaa" title="Same as TT_DologinEx() but without the option to specify szClientName. Kept for backwards compatibili...">TT_DoLogin</a> command can be issued. Always ensure to call <a class="el" href="group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e" title="Disconnect from the server.">TT_Disconnect</a> before attempting to create a new connection with <a class="el" href="group__connectivity.html#ga0dd61484f6e2177ff96e3f5027f99861" title="Connect to a server.">TT_Connect</a>.</p>
... (67 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2484:    TEAMTALKDLL_API INT32 TT_DoLogin(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2491:    TEAMTALKDLL_API INT32 TT_DoLoginEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:252:            ffi::api().TT_DoLoginEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoLoginEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1000:<p>On top of the system-ID restriction it's now also possible to do a check on the client name connecting by using the extended <a class="el" href="group__commands.html#ga3f195e405b5598d86663975dce37e401" title="Logon to a server.">TT_DoLoginEx()</a>. When a client tries to connect to a TeamTalk server you can then check the <code>szClientName</code> property of <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a> to ensure only your client application is allowed to connect.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1030:<p>Added <code>szClientName</code> to <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a>-struct. The client name is specified in <a class="el" href="group__commands.html#ga3f195e405b5598d86663975dce37e401" title="Logon to a server.">TT_DoLoginEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:211:<li>TT_DoLoginEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:265:<li>TT_DoLoginEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:5:    [ "TT_DoLoginEx", "group__commands.html#ga3f195e405b5598d86663975dce37e401", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:103:<tr class="memitem:ga3f195e405b5598d86663975dce37e401"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga3f195e405b5598d86663975dce37e401">TT_DoLoginEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szNickname, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szUsername, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szPassword, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szClientName)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:297:<h2 class="memtitle"><span class="permalink"><a href="#ga3f195e405b5598d86663975dce37e401">&#9670;&nbsp;</a></span>TT_DoLoginEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:303:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoLoginEx </td>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2491:    TEAMTALKDLL_API INT32 TT_DoLoginEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:252:            ffi::api().TT_DoLoginEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoLogout`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:214:<li>TT_DoLogout()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:268:<li>TT_DoLogout()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:346:   #TT_DoLogout could not be performed because client
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:6:    [ "TT_DoLogout", "group__commands.html#gad55b94ebda5761e78687ce68e1cfafe6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:401:<p>A response to <a class="el" href="group__commands.html#gad55b94ebda5761e78687ce68e1cfafe6" title="Logout of the server.">TT_DoLogout</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:404:<li><a class="el" href="struct_t_t_message.html#afe01294f7577f7dae9c559145c73aa86" title="Specifies which member to access in the union.">TTMessage.ttType</a> <a class="el" href="group__events.html#ggaac48a84de49d548073a3f81276a44d57a7528fb8469eed65b4e48c5f96c17c1f1">__NONE</a> <dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gad55b94ebda5761e78687ce68e1cfafe6" title="Logout of the server.">TT_DoLogout</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:428:<p>This event is called when a user logs out with <a class="el" href="group__commands.html#gad55b94ebda5761e78687ce68e1cfafe6" title="Logout of the server.">TT_DoLogout</a> or disconnects with <a class="el" href="group__connectivity.html#ga243d6ae41b50422e3f6f9b1046d26c3e" title="Disconnect from the server.">TT_Disconnect</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:434:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gad55b94ebda5761e78687ce68e1cfafe6" title="Logout of the server.">TT_DoLogout</a> </dd>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2498:    TEAMTALKDLL_API INT32 TT_DoLogout(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:263:        unsafe { ffi::api().TT_DoLogout(ptr) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoMakeChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:612:<p>Use <a class="el" href="group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c" title="Make a new channel on the server.">TT_DoMakeChannel()</a> or <a class="el" href="group__commands.html#ga8b15a791c8034ec640a4a03435704333" title="Create a new channel and join it.">TT_DoJoinChannel()</a> to create a hidden channel.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2342:<li>When creating a <a class="el" href="struct_channel.html" title="A struct containing the properties of a channel.">Channel</a> using <a class="el" href="group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c" title="Make a new channel on the server.">TT_DoMakeChannel()</a> or <a class="el" href="group__commands.html#ga8b15a791c8034ec640a4a03435704333" title="Create a new channel and join it.">TT_DoJoinChannel()</a> it is now possible to specify an audio configuration so all users will speak at the same volume level.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:217:<li>TT_DoMakeChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:271:<li>TT_DoMakeChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:21:    [ "TT_DoMakeChannel", "group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:250:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c" title="Make a new channel on the server.">TT_DoMakeChannel</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:259:   invalid channel password. #TT_DoMakeChannel can also cause
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:294:   @see TT_DoMakeChannel
... (20 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2570:    TEAMTALKDLL_API INT32 TT_DoMakeChannel(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:128:        unsafe { ffi::api().TT_DoMakeChannel(self.ptr.0, &channel.to_ffi()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoMoveUser`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:220:<li>TT_DoMoveUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:274:<li>TT_DoMoveUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:160:<tr class="memitem:ga5bb3a048735f291f0338a2e847913895"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga5bb3a048735f291f0338a2e847913895">TT_DoMoveUser</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN INT32 nChannelID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1331:<h2 class="memtitle"><span class="permalink"><a href="#ga5bb3a048735f291f0338a2e847913895">&#9670;&nbsp;</a></span>TT_DoMoveUser()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1337:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoMoveUser </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1381:<dl class="section see"><dt>See also</dt><dd>TT_DoMoveUserByID </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:24:    [ "TT_DoMoveUser", "group__commands.html#ga5bb3a048735f291f0338a2e847913895", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:296:   @see TT_DoMoveUser
... (11 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2582:    TEAMTALKDLL_API INT32 TT_DoMoveUser(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:152:        unsafe { ffi::api().TT_DoMoveUser(self.ptr.0, user_id.0, channel_id.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoNewUserAccount`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2430:<li>Call <a class="el" href="group__commands.html#ga5ff5291f5d04f19a5e9e674d98ff9ecd" title="Issue command to create a new user account on the server.">TT_DoNewUserAccount</a> to create a new user account. The user doing this must be administrator, i.e. <a class="el" href="group__users.html#gga196c5ae3c192f069339f2966656f6fe9a70ed800bccef95be29f3ae2d75ab8c1d" title="A user with administrator privileges.">USERTYPE_ADMIN</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:223:<li>TT_DoNewUserAccount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:169:<tr class="memitem:ga5ff5291f5d04f19a5e9e674d98ff9ecd"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga5ff5291f5d04f19a5e9e674d98ff9ecd">TT_DoNewUserAccount</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="struct_user_account.html">UserAccount</a> *lpUserAccount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1488:<h2 class="memtitle"><span class="permalink"><a href="#ga5ff5291f5d04f19a5e9e674d98ff9ecd">&#9670;&nbsp;</a></span>TT_DoNewUserAccount()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1494:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoNewUserAccount </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1582:<a class="el" href="group__commands.html#ga5ff5291f5d04f19a5e9e674d98ff9ecd" title="Issue command to create a new user account on the server.">TT_DoNewUserAccount</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:277:<li>TT_DoNewUserAccount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:27:    [ "TT_DoNewUserAccount", "group__commands.html#ga5ff5291f5d04f19a5e9e674d98ff9ecd", null ],
... (17 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2596:    TEAMTALKDLL_API INT32 TT_DoNewUserAccount(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:474:        unsafe { ffi::api().TT_DoNewUserAccount(self.ptr.0, &account.to_ffi()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoPing`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1682:<li><a class="el" href="group__commands.html#gac1c27004f9dc514905e32e1de3fce0bb" title="Ping server and wait for server to reply.">TT_DoPing()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1879:<li>Use <a class="el" href="group__commands.html#gac1c27004f9dc514905e32e1de3fce0bb" title="Ping server and wait for server to reply.">TT_DoPing()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:226:<li>TT_DoPing()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:280:<li>TT_DoPing()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:198:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gac1c27004f9dc514905e32e1de3fce0bb" title="Ping server and wait for server to reply.">TT_DoPing()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:644:<p>After calling <a class="el" href="group__connectivity.html#gadf4cc840006b7c4f49caac2f63ad3e5f" title="Update the client instance&#39;s default keep alive settings.">TT_SetClientKeepAlive()</a> it is recommended doing a <a class="el" href="group__commands.html#gac1c27004f9dc514905e32e1de3fce0bb" title="Ping server and wait for server to reply.">TT_DoPing()</a> since all TCP and UDP keep alive timers will be restarted.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:645:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gac1c27004f9dc514905e32e1de3fce0bb" title="Ping server and wait for server to reply.">TT_DoPing()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:680:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#gac1c27004f9dc514905e32e1de3fce0bb" title="Ping server and wait for server to reply.">TT_DoPing()</a> </dd>
... (16 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2481:    TEAMTALKDLL_API INT32 TT_DoPing(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:98:        unsafe { ffi::api().TT_DoPing(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoQueryServerStats`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1439:<li>Response to <a class="el" href="group__commands.html#gaff8df90f0587776fc8ff586191e0f71d" title="Get the server&#39;s current statistics.">TT_DoQueryServerStats()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2290:<p>Administrators can now query a server statistics using the command <a class="el" href="group__commands.html#gaff8df90f0587776fc8ff586191e0f71d" title="Get the server&#39;s current statistics.">TT_DoQueryServerStats()</a> and thereby get an overview of bandwidth usage.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2353:<li><a class="el" href="group__commands.html#gaff8df90f0587776fc8ff586191e0f71d" title="Get the server&#39;s current statistics.">TT_DoQueryServerStats()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2358:<li>After a successful call to <a class="el" href="group__commands.html#gaff8df90f0587776fc8ff586191e0f71d" title="Get the server&#39;s current statistics.">TT_DoQueryServerStats()</a> extract the server's statistics in a <a class="el" href="struct_server_statistics.html" title="A struct containing the server&#39;s statistics, i.e. bandwidth usage and user activity.">ServerStatistics</a> struct.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:229:<li>TT_DoQueryServerStats()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:283:<li>TT_DoQueryServerStats()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:37:    [ "TT_DoQueryServerStats", "group__commands.html#gaff8df90f0587776fc8ff586191e0f71d", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:199:<tr class="memitem:gaff8df90f0587776fc8ff586191e0f71d"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gaff8df90f0587776fc8ff586191e0f71d">TT_DoQueryServerStats</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2640:    TEAMTALKDLL_API INT32 TT_DoQueryServerStats(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:93:        unsafe { ffi::api().TT_DoQueryServerStats(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoQuit`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:232:<li>TT_DoQuit()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:38:    [ "TT_DoQuit", "group__commands.html#ga517ceff5ba413b461c9df4577137313c", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:286:<li>TT_DoQuit()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:202:<tr class="memitem:ga517ceff5ba413b461c9df4577137313c"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga517ceff5ba413b461c9df4577137313c">TT_DoQuit</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:2011:<h2 class="memtitle"><span class="permalink"><a href="#ga517ceff5ba413b461c9df4577137313c">&#9670;&nbsp;</a></span>TT_DoQuit()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:2017:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoQuit </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:78:  ['tt_5fdoquit_742',['TT_DoQuit',['../group__commands.html#ga517ceff5ba413b461c9df4577137313c',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.js:678:    [ "TT_DoQuit", "group__commands.html#ga517ceff5ba413b461c9df4577137313c", null ],
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2643:    TEAMTALKDLL_API INT32 TT_DoQuit(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:123:        unsafe { ffi::api().TT_DoQuit(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoRecvFile`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:235:<li>TT_DoRecvFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:289:<li>TT_DoRecvFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:17:    [ "TT_DoRecvFile", "group__commands.html#gad5b7fff4d1bf97aa6ed080da4f8d318c", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:139:<tr class="memitem:gad5b7fff4d1bf97aa6ed080da4f8d318c"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gad5b7fff4d1bf97aa6ed080da4f8d318c">TT_DoRecvFile</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID, IN INT32 nFileID, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szLocalFilePath)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:951:<h2 class="memtitle"><span class="permalink"><a href="#gad5b7fff4d1bf97aa6ed080da4f8d318c">&#9670;&nbsp;</a></span>TT_DoRecvFile()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:957:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoRecvFile </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:234:<p>Call <a class="el" href="group__commands.html#ga41238b390d4fa02bf6cbc01ea043c971" title="Send a file to the specified channel.">TT_DoSendFile</a> to upload a file and <a class="el" href="group__commands.html#gad5b7fff4d1bf97aa6ed080da4f8d318c" title="Download a file from the specified channel.">TT_DoRecvFile</a> to download a file. Only users who have a <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> on the server are allowed to upload files. There is no limit on the maximum number of file transfers but it is advised to queue file transfers so the file transfers do no affect server performance.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:300:   @see TT_DoRecvFile 
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2549:    TEAMTALKDLL_API INT32 TT_DoRecvFile(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:56:            ffi::api().TT_DoRecvFile(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoRemoveChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:238:<li>TT_DoRemoveChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:292:<li>TT_DoRemoveChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:157:<tr class="memitem:gab4d7e1df7ca04ced7b09d00621d08b52"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gab4d7e1df7ca04ced7b09d00621d08b52">TT_DoRemoveChannel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1284:<h2 class="memtitle"><span class="permalink"><a href="#gab4d7e1df7ca04ced7b09d00621d08b52">&#9670;&nbsp;</a></span>TT_DoRemoveChannel()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1290:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoRemoveChannel </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:295:   @see TT_DoRemoveChannel
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:364:   @see TT_DoRemoveChannel
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:229:<p>With <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679a9dd9ff869605b04e35796a3075e48fed" title="User is allowed to create permanent channels which are stored in the server&#39;s configuration file.">USERRIGHT_MODIFY_CHANNELS</a> the method <a class="el" href="group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c" title="Make a new channel on the server.">TT_DoMakeChannel()</a> can be used to create a new channel and any existing channel can be updated using <a class="el" href="group__commands.html#ga87a3bc3856d98b33dadc80b0d9e5306b" title="Update a channel&#39;s properties.">TT_DoUpdateChannel()</a> and removed by calling <a class="el" href="group__commands.html#gab4d7e1df7ca04ced7b09d00621d08b52" title="Remove a channel from a server.">TT_DoRemoveChannel()</a>. Basically <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679a9dd9ff869605b04e35796a3075e48fed" title="User is allowed to create permanent channels which are stored in the server&#39;s configuration file.">USERRIGHT_MODIFY_CHANNELS</a> gives unrestricted access to all channels on the server. Also seeing passwords of all channels on the server.</p>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2578:    TEAMTALKDLL_API INT32 TT_DoRemoveChannel(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:144:        unsafe { ffi::api().TT_DoRemoveChannel(self.ptr.0, id.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoSaveConfig`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:241:<li>TT_DoSaveConfig()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:295:<li>TT_DoSaveConfig()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:36:    [ "TT_DoSaveConfig", "group__commands.html#gaf2594a5cd347ff8737005eefe49edc45", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:196:<tr class="memitem:gaf2594a5cd347ff8737005eefe49edc45"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#gaf2594a5cd347ff8737005eefe49edc45">TT_DoSaveConfig</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1943:<h2 class="memtitle"><span class="permalink"><a href="#gaf2594a5cd347ff8737005eefe49edc45">&#9670;&nbsp;</a></span>TT_DoSaveConfig()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1949:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoSaveConfig </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:298:   @see TT_DoSaveConfig
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:81:  ['tt_5fdosaveconfig_745',['TT_DoSaveConfig',['../group__commands.html#gaf2594a5cd347ff8737005eefe49edc45',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2637:    TEAMTALKDLL_API INT32 TT_DoSaveConfig(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:80:        unsafe { ffi::api().TT_DoSaveConfig(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoSendFile`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:244:<li>TT_DoSendFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:298:<li>TT_DoSendFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:234:<p>Call <a class="el" href="group__commands.html#ga41238b390d4fa02bf6cbc01ea043c971" title="Send a file to the specified channel.">TT_DoSendFile</a> to upload a file and <a class="el" href="group__commands.html#gad5b7fff4d1bf97aa6ed080da4f8d318c" title="Download a file from the specified channel.">TT_DoRecvFile</a> to download a file. Only users who have a <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> on the server are allowed to upload files. There is no limit on the maximum number of file transfers but it is advised to queue file transfers so the file transfers do no affect server performance.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:16:    [ "TT_DoSendFile", "group__commands.html#ga41238b390d4fa02bf6cbc01ea043c971", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:299:   @see TT_DoSendFile 
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:309:<pre class="fragment">   #TT_DoSendFile was not allowed because there's not enough
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:341:<pre class="fragment">    @see TT_DoSendFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:367:   @see TT_DoSendFile
... (15 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2544:    TEAMTALKDLL_API INT32 TT_DoSendFile(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:47:        unsafe { ffi::api().TT_DoSendFile(self.ptr.0, channel_id.0, local_path.tt().as_ptr()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoSubscribe`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:775:<p>Now it's possible to record multiple channels at the same time using <a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113" title="Store audio conversations from a specific channel into a single file.">TT_StartRecordingMuxedAudioFileEx()</a>. Note that in order to get audio from channels where the TeamTalk instance is currently not participating requires the use of <a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d" title="Subscribe to user events and/or data.">TT_DoSubscribe()</a> and <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443ba304cea831425da3b9c0816dc96ae5015" title="Intercept all voice sent by a user. Only user-type USERTYPE_ADMIN can do this. By enabling this subsc...">SUBSCRIBE_INTERCEPT_VOICE</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2642:<li>Call <a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d" title="Subscribe to user events and/or data.">TT_DoSubscribe</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:247:<li>TT_DoSubscribe()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:301:<li>TT_DoSubscribe()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:19:    [ "TT_DoSubscribe", "group__commands.html#ga54fb7c84fa6707f11f385709456ae94d", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:145:<tr class="memitem:ga54fb7c84fa6707f11f385709456ae94d"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d">TT_DoSubscribe</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__users.html#ga56f483fd85341c1483c7f1cdf93058bc">Subscriptions</a> uSubscriptions)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1074:<h2 class="memtitle"><span class="permalink"><a href="#ga54fb7c84fa6707f11f385709456ae94d">&#9670;&nbsp;</a></span>TT_DoSubscribe()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1080:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoSubscribe </td>
... (24 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2560:    TEAMTALKDLL_API INT32 TT_DoSubscribe(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:490:        unsafe { ffi::api().TT_DoSubscribe(self.ptr.0, user_id.0, mask.raw()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoTextMessage`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:608:<p>Previously it was only possible to control <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dae5064c6cd0444d6e4f46598eaf4fb018" title="Voice stream type which is audio recorded from a sound input device.">STREAMTYPE_VOICE</a>, <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1da4bf8c942fa3454c6d2a6b94938b9e3a0" title="Shortcut to allow both audio and video media files.">STREAMTYPE_MEDIAFILE</a>, <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1da761a55095775609174bfdef145651f97" title="Video capture stream type which is video recorded from a webcam.">STREAMTYPE_VIDEOCAPTURE</a> and <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dab7b638a607c20ffe34ed4cb15d062859" title="Desktop window stream type which is a window (or bitmap) being transmitted.">STREAMTYPE_DESKTOP</a> in a classroom using <code>transmitUsers</code> on <a class="el" href="struct_channel.html" title="A struct containing the properties of a channel.">Channel</a>-struct. Now <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1daeced325cc6f9879b831f25353baffaa1" title="Channel text messages as stream type.">STREAMTYPE_CHANNELMSG</a> has been added so it's also possible to enable/disable channel text messages sent through <a class="el" href="group__commands.html#ga862ce0557a142b61f06fd231d2120187" title="Send a text message to either a user or a channel.">TT_DoTextMessage()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2554:<li>Call <a class="el" href="group__commands.html#ga862ce0557a142b61f06fd231d2120187" title="Send a text message to either a user or a channel.">TT_DoTextMessage</a> with message type <a class="el" href="group__users.html#gga35cce2235269395f80a27921aea1e5c4ac22fd54615479ddf1526580840765d4b" title="A User to user text message. A message of this type can be sent across channels.">MSGTYPE_USER</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2558:<li>Call <a class="el" href="group__commands.html#ga862ce0557a142b61f06fd231d2120187" title="Send a text message to either a user or a channel.">TT_DoTextMessage</a> with message type <a class="el" href="group__users.html#gga35cce2235269395f80a27921aea1e5c4aa4ea3cd27e7a24e5a79c7c295e53e135" title="A User to channel text message. Users of type USERTYPE_DEFAULT can only send this text message to the...">MSGTYPE_CHANNEL</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:250:<li>TT_DoTextMessage()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:124:<tr class="memitem:ga862ce0557a142b61f06fd231d2120187"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga862ce0557a142b61f06fd231d2120187">TT_DoTextMessage</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="struct_text_message.html">TextMessage</a> *lpTextMessage)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:656:<h2 class="memtitle"><span class="permalink"><a href="#ga862ce0557a142b61f06fd231d2120187">&#9670;&nbsp;</a></span>TT_DoTextMessage()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:662:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoTextMessage </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:12:    [ "TT_DoTextMessage", "group__commands.html#ga862ce0557a142b61f06fd231d2120187", null ],
... (23 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2522:    TEAMTALKDLL_API INT32 TT_DoTextMessage(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:280:        unsafe { ffi::api().TT_DoTextMessage(ptr, message) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoUnBanUser`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:940:<li>New function <a class="el" href="group__commands.html#ga05d90efbcad0d492971f2c7c2b183c2f" title="Unban the properties specified in BannedUser.">TT_DoUnBanUserEx()</a> for removing a bans.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:253:<li>TT_DoUnBanUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:256:<li>TT_DoUnBanUserEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:302:   @see TT_DoUnBanUser
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:390:<pre class="fragment">   #TT_DoUnBanUser failed because there is no banned
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:307:<li>TT_DoUnBanUser()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:310:<li>TT_DoUnBanUserEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:33:    [ "TT_DoUnBanUser", "group__commands.html#ga5394198eed28184bc72e6aa9660d28cf", null ],
... (23 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2623:    TEAMTALKDLL_API INT32 TT_DoUnBanUser(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2627:    TEAMTALKDLL_API INT32 TT_DoUnBanUserEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:345:        unsafe { ffi::api().TT_DoUnBanUser(self.ptr.0, ip.tt().as_ptr(), channel_id.0) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:426:        unsafe { ffi::api().TT_DoUnBanUserEx(self.ptr.0, &banned_user.to_ffi()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoUnBanUserEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:940:<li>New function <a class="el" href="group__commands.html#ga05d90efbcad0d492971f2c7c2b183c2f" title="Unban the properties specified in BannedUser.">TT_DoUnBanUserEx()</a> for removing a bans.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:256:<li>TT_DoUnBanUserEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.js:34:    [ "TT_DoUnBanUserEx", "group__commands.html#ga05d90efbcad0d492971f2c7c2b183c2f", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:190:<tr class="memitem:ga05d90efbcad0d492971f2c7c2b183c2f"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga05d90efbcad0d492971f2c7c2b183c2f">TT_DoUnBanUserEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="struct_banned_user.html">BannedUser</a> *lpBannedUser)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1839:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#ga05d90efbcad0d492971f2c7c2b183c2f" title="Unban the properties specified in BannedUser.">TT_DoUnBanUserEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1850:<h2 class="memtitle"><span class="permalink"><a href="#ga05d90efbcad0d492971f2c7c2b183c2f">&#9670;&nbsp;</a></span>TT_DoUnBanUserEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1856:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoUnBanUserEx </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:310:<li>TT_DoUnBanUserEx()
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2627:    TEAMTALKDLL_API INT32 TT_DoUnBanUserEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:426:        unsafe { ffi::api().TT_DoUnBanUserEx(self.ptr.0, &banned_user.to_ffi()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoUnsubscribe`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2646:<li>Call <a class="el" href="group__commands.html#ga2373fded851f55d97e94903e54cd900d" title="Unsubscribe to user events/data. This can be used to ignore messages or voice data from a specific us...">TT_DoUnsubscribe</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:259:<li>TT_DoUnsubscribe()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:313:<li>TT_DoUnsubscribe()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2024:<p>To stop receiving audio from a user call <a class="el" href="group__commands.html#ga2373fded851f55d97e94903e54cd900d" title="Unsubscribe to user events/data. This can be used to ignore messages or voice data from a specific us...">TT_DoUnsubscribe</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2361:<p>To stop receiving audio from a user call <a class="el" href="group__commands.html#ga2373fded851f55d97e94903e54cd900d" title="Unsubscribe to user events/data. This can be used to ignore messages or voice data from a specific us...">TT_DoUnsubscribe</a> with <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443ba76a67a1ec77800dbf256b2d0f45cb92a" title="Subscribing to STREAMTYPE_VOICE.">SUBSCRIBE_VOICE</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:256:<p>When logging on to a server the local client instance will by default subscribe to user messages, channel messages, broadcast messages, audio data and video data from all users. If, however, a client wants to stop receiving e.g. audio from a user, he can call <a class="el" href="group__commands.html#ga2373fded851f55d97e94903e54cd900d" title="Unsubscribe to user events/data. This can be used to ignore messages or voice data from a specific us...">TT_DoUnsubscribe</a> along with the user ID and the <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443ba76a67a1ec77800dbf256b2d0f45cb92a" title="Subscribing to STREAMTYPE_VOICE.">SUBSCRIBE_VOICE</a>-flag to tell the server that he no longer wants to receive audio from that user. The server will then respond with the event <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a985bf23cebd50f38e5660ca12a733bc1" title="User changed properties.">CLIENTEVENT_CMD_USER_UPDATE</a> and the <em>uLocalSubscriptions</em> member of <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a> will have the <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443ba76a67a1ec77800dbf256b2d0f45cb92a" title="Subscribing to STREAMTYPE_VOICE.">SUBSCRIBE_VOICE</a>-flag removed. At the remote user the <em>uPeerSubscriptions</em> member will be changed. Subscribe/unsubscribe can also be done for user, channel and broadcast messages and video data. The function <a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d" title="Subscribe to user events and/or data.">TT_DoSubscribe</a> can be used to revert unsubscriptions.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:313:<p>By calling <a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d" title="Subscribe to user events and/or data.">TT_DoSubscribe</a> and <a class="el" href="group__commands.html#ga2373fded851f55d97e94903e54cd900d" title="Unsubscribe to user events/data. This can be used to ignore messages or voice data from a specific us...">TT_DoUnsubscribe</a> the local client instance can tell the server (and thereby remote users) what he is willing to accept from other users.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:317:<a class="el" href="group__commands.html#ga2373fded851f55d97e94903e54cd900d" title="Unsubscribe to user events/data. This can be used to ignore messages or voice data from a specific us...">TT_DoUnsubscribe</a> </dd></dl>
... (18 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2565:    TEAMTALKDLL_API INT32 TT_DoUnsubscribe(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:498:        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr.0, user_id.0, mask.raw()) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:506:        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr.0, user_id.0, Subscriptions::ALL) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:514:        unsafe { ffi::api().TT_DoUnsubscribe(self.ptr.0, 0, Subscriptions::ALL) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoUpdateChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2346:<li>This is a new channel-type where a channel operator or administrator can select which users are allowed to talk and send video to a channel. Use <em>voiceUsers</em> and <em>videoUsers</em> members of <a class="el" href="struct_channel.html" title="A struct containing the properties of a channel.">Channel</a> to select who is allowed to transmit then afterwards call <a class="el" href="group__commands.html#ga87a3bc3856d98b33dadc80b0d9e5306b" title="Update a channel&#39;s properties.">TT_DoUpdateChannel()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:262:<li>TT_DoUpdateChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:229:<p>With <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679a9dd9ff869605b04e35796a3075e48fed" title="User is allowed to create permanent channels which are stored in the server&#39;s configuration file.">USERRIGHT_MODIFY_CHANNELS</a> the method <a class="el" href="group__commands.html#ga79bdd82c6fb510747c57961e5fe0d29c" title="Make a new channel on the server.">TT_DoMakeChannel()</a> can be used to create a new channel and any existing channel can be updated using <a class="el" href="group__commands.html#ga87a3bc3856d98b33dadc80b0d9e5306b" title="Update a channel&#39;s properties.">TT_DoUpdateChannel()</a> and removed by calling <a class="el" href="group__commands.html#gab4d7e1df7ca04ced7b09d00621d08b52" title="Remove a channel from a server.">TT_DoRemoveChannel()</a>. Basically <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679a9dd9ff869605b04e35796a3075e48fed" title="User is allowed to create permanent channels which are stored in the server&#39;s configuration file.">USERRIGHT_MODIFY_CHANNELS</a> gives unrestricted access to all channels on the server. Also seeing passwords of all channels on the server.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:443:<p>To change the properties of a channel call <a class="el" href="group__commands.html#ga87a3bc3856d98b33dadc80b0d9e5306b" title="Update a channel&#39;s properties.">TT_DoUpdateChannel()</a>. Note that <em>audiocodec</em> cannot be changed if the channel has users.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:526:<p>For a user to transmit audio or video to this type of channel the channel operator must add the user's ID to <code>transmitUsers</code> in the <a class="el" href="struct_channel.html" title="A struct containing the properties of a channel.">Channel</a> struct and call <a class="el" href="group__commands.html#ga87a3bc3856d98b33dadc80b0d9e5306b" title="Update a channel&#39;s properties.">TT_DoUpdateChannel()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:154:<tr class="memitem:ga87a3bc3856d98b33dadc80b0d9e5306b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga87a3bc3856d98b33dadc80b0d9e5306b">TT_DoUpdateChannel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="struct_channel.html">Channel</a> *lpChannel)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1227:<h2 class="memtitle"><span class="permalink"><a href="#ga87a3bc3856d98b33dadc80b0d9e5306b">&#9670;&nbsp;</a></span>TT_DoUpdateChannel()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1233:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoUpdateChannel </td>
... (15 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2574:    TEAMTALKDLL_API INT32 TT_DoUpdateChannel(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:136:        unsafe { ffi::api().TT_DoUpdateChannel(self.ptr.0, &channel.to_ffi()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_DoUpdateServer`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:548:<p>It's now possible to change log levels while the TeamTalk server is running. Setup <code>uServerLogEvents</code> in <a class="el" href="struct_server_properties.html" title="A struct containing the properties of the server&#39;s settings.">ServerProperties</a> and issue <a class="el" href="group__commands.html#ga60750fad98b67bf156f754268ef72889" title="Update server properties.">TT_DoUpdateServer()</a>. <code>uServerLogEvents</code> is based on <a class="el" href="group__server.html#gab0600eb7faa5c10a1013bbd5d342fe99" title="Events that are logged by the server, i.e. written to server&#39;s log file.">ServerLogEvent</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:265:<li>TT_DoUpdateServer()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:319:<li>TT_DoUpdateServer()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:163:<tr class="memitem:ga60750fad98b67bf156f754268ef72889"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__commands.html#ga60750fad98b67bf156f754268ef72889">TT_DoUpdateServer</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="struct_server_properties.html">ServerProperties</a> *lpServerProperties)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1386:<h2 class="memtitle"><span class="permalink"><a href="#ga60750fad98b67bf156f754268ef72889">&#9670;&nbsp;</a></span>TT_DoUpdateServer()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1392:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_DoUpdateServer </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1972:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__commands.html#ga60750fad98b67bf156f754268ef72889" title="Update server properties.">TT_DoUpdateServer</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:297:   @see TT_DoUpdateServer
... (14 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2587:    TEAMTALKDLL_API INT32 TT_DoUpdateServer(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:72:        unsafe { ffi::api().TT_DoUpdateServer(self.ptr.0, &props.to_ffi()) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Enable3DSoundPositioning`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2526:<li>Call <a class="el" href="group__sounddevices.html#ga4c03637a85783707205cf4f40c770136" title="Enable automatically position users using 3D-sound.">TT_Enable3DSoundPositioning</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:322:<li>TT_Enable3DSoundPositioning()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:268:<li>TT_Enable3DSoundPositioning()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:274:<a class="el" href="group__sounddevices.html#ga4c03637a85783707205cf4f40c770136" title="Enable automatically position users using 3D-sound.">TT_Enable3DSoundPositioning</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:108:    [ "TT_Enable3DSoundPositioning", "group__sounddevices.html#ga4c03637a85783707205cf4f40c770136", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:303:<tr class="memitem:ga4c03637a85783707205cf4f40c770136"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga4c03637a85783707205cf4f40c770136">TT_Enable3DSoundPositioning</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEnable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2037:<h2 class="memtitle"><span class="permalink"><a href="#ga4c03637a85783707205cf4f40c770136">&#9670;&nbsp;</a></span>TT_Enable3DSoundPositioning()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2043:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Enable3DSoundPositioning </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2111:    TEAMTALKDLL_API TTBOOL TT_Enable3DSoundPositioning(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:261:            ffi::api().TT_Enable3DSoundPositioning(self.ptr.0, if enable { 1 } else { 0 }) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_EnableAudioBlockEvent`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:589:<p>To retrieve <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> with mixed audio use <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a> or <a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEventEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:712:<li>New macro <a class="el" href="group__sounddevices.html#ga66e21f86ae0f0dd1af6f194d3879d1f5" title="User ID passed to TT_EnableAudioBlockEvent() in order to receive AudioBlock when voice transmission i...">TT_LOCAL_TX_USERID</a> for only getting audio during voice transmission when using <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:722:<li>New function <a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEventEx()</a> for requesting <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> with the specified <a class="el" href="struct_audio_format.html" title="Struct describing the audio format used by a media file.">AudioFormat</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:763:<p>Previously <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a> could only be used to access audio from a single user. Using <a class="el" href="group__sounddevices.html#gaae84cd30592b71d2b43c37b7e414ca2e" title="User ID used to identify muxed audio that has been mixed into a single stream.">TT_MUXED_USERID</a> now makes it possible to access the audio stream where all users' audio streams have been mixed together. Basically the same as recording all conversations to a single file using <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:789:<li>New macro <a class="el" href="group__sounddevices.html#gaae84cd30592b71d2b43c37b7e414ca2e" title="User ID used to identify muxed audio that has been mixed into a single stream.">TT_MUXED_USERID</a> for <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a>.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:793:<li>New macro <a class="el" href="group__sounddevices.html#ga66ea6823fd342421e8a7df589f731a8a" title="User ID passed to TT_EnableAudioBlockEvent() in order to receive AudioBlock directly from sound input...">TT_LOCAL_USERID</a> for <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1770:<li><a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2094:<p>The raw audio, which has been playing when a user is talking, can now be accessed by calling <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a>. The event <code>WM_TEAMTALK_USER_AUDIOBLOCK</code> is triggered when a new <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> is available.</p>
... (53 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2118:    TEAMTALKDLL_API TTBOOL TT_EnableAudioBlockEvent(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2124:    TEAMTALKDLL_API TTBOOL TT_EnableAudioBlockEventEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:304:            ffi::api().TT_EnableAudioBlockEvent(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:337:            ffi::api().TT_EnableAudioBlockEventEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_EnableAudioBlockEventEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:99:<dd><a class="anchor" id="_deprecated000009"></a>Use <a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEventEx()</a>. </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:589:<p>To retrieve <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> with mixed audio use <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a> or <a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEventEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:722:<li>New function <a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEventEx()</a> for requesting <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> with the specified <a class="el" href="struct_audio_format.html" title="Struct describing the audio format used by a media file.">AudioFormat</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:274:<li>TT_EnableAudioBlockEventEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:328:<li>TT_EnableAudioBlockEventEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:312:<tr class="memitem:gabd6586373d22190a78acb6368144a7aa"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa">TT_EnableAudioBlockEventEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga6c16695e0994a2ee32d4e93c15daeaaa">StreamTypes</a> uStreamTypes, IN const <a class="el" href="struct_audio_format.html">AudioFormat</a> *lpAudioFormat, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEnable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2144:<dl class="deprecated"><dt><b><a class="el" href="deprecated.html#_deprecated000009">Deprecated:</a></b></dt><dd>Use <a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEventEx()</a>.</dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2148:    <tr><td class="paramname">nUserID</td><td>See description in <a class="el" href="group__sounddevices.html#gabd6586373d22190a78acb6368144a7aa" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEventEx()</a> </td></tr>
... (11 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2124:    TEAMTALKDLL_API TTBOOL TT_EnableAudioBlockEventEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:337:            ffi::api().TT_EnableAudioBlockEventEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_EnableVoiceActivation`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:534:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__transmission.html#gafbaf582493a7c808d44d2020b13c812f" title="Enable voice activation.">TT_EnableVoiceActivation()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:277:<li>TT_EnableVoiceActivation()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:331:<li>TT_EnableVoiceActivation()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:261:<a class="el" href="group__transmission.html#gafbaf582493a7c808d44d2020b13c812f" title="Enable voice activation.">TT_EnableVoiceActivation</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:266:<a class="el" href="group__transmission.html#gafbaf582493a7c808d44d2020b13c812f" title="Enable voice activation.">TT_EnableVoiceActivation()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:544:<p>Either through <a class="el" href="group__transmission.html#gafbaf582493a7c808d44d2020b13c812f" title="Enable voice activation.">TT_EnableVoiceActivation()</a> or <a class="el" href="group__transmission.html#ga8ef1203cd2998908c95761c2621b573c" title="Start/stop transmitting of voice data from sound input.">TT_EnableVoiceTransmission()</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:20:    [ "TT_EnableVoiceActivation", "group__transmission.html#gafbaf582493a7c808d44d2020b13c812f", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:135:<tr class="memitem:gafbaf582493a7c808d44d2020b13c812f"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#gafbaf582493a7c808d44d2020b13c812f">TT_EnableVoiceActivation</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEnable)</td></tr>
... (14 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2143:    TEAMTALKDLL_API TTBOOL TT_EnableVoiceActivation(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:205:        unsafe { ffi::api().TT_EnableVoiceActivation(self.ptr.0, if enable { 1 } else { 0 }) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_EnableVoiceTransmission`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1357:<li><code>USERRIGHT_FORWARD_AUDIO</code> replaced by <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679afc11323082ea6f7667a9f4368885b058" title="Users are allowed to forward audio packets through server. TT_EnableVoiceTransmission()">USERRIGHT_TRANSMIT_VOICE</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1638:<li><a class="el" href="group__transmission.html#ga8ef1203cd2998908c95761c2621b573c" title="Start/stop transmitting of voice data from sound input.">TT_EnableVoiceTransmission()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1839:<li>Use <a class="el" href="group__transmission.html#ga8ef1203cd2998908c95761c2621b573c" title="Start/stop transmitting of voice data from sound input.">TT_EnableVoiceTransmission()</a> or <a class="el" href="group__transmission.html#ga1e76ef6ae7f72331dff1dbd9880baaa4" title="Start transmitting from video capture device.">TT_StartVideoCaptureTransmission()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:280:<li>TT_EnableVoiceTransmission()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:334:<li>TT_EnableVoiceTransmission()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:165:<p>If the server should not allow clients to forward audio and video packets the <em>uUserRights</em> member of <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> must disable <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679afc11323082ea6f7667a9f4368885b058" title="Users are allowed to forward audio packets through server. TT_EnableVoiceTransmission()">USERRIGHT_TRANSMIT_VOICE</a> and <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679ab57030f23de524c7fdf647c847c960fb" title="User is allowed to forward video packets through server. TT_StartVideoCaptureTransmission()">USERRIGHT_TRANSMIT_VIDEOCAPTURE</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:533:<tr><td class="fieldname"><a id="ggaa62615f8034ace22e5dd6dfa6778e679afc11323082ea6f7667a9f4368885b058"></a>USERRIGHT_TRANSMIT_VOICE&#160;</td><td class="fielddoc"><p>Users are allowed to forward audio packets through server. <a class="el" href="group__transmission.html#ga8ef1203cd2998908c95761c2621b573c" title="Start/stop transmitting of voice data from sound input.">TT_EnableVoiceTransmission()</a> </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:162:<p>A hotkey can e.g. be used as a push-to-talk key combination. When the hotkey becomes active call <a class="el" href="group__transmission.html#ga8ef1203cd2998908c95761c2621b573c" title="Start/stop transmitting of voice data from sound input.">TT_EnableVoiceTransmission()</a>.</p>
... (20 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2139:    TEAMTALKDLL_API TTBOOL TT_EnableVoiceTransmission(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:199:            ffi::api().TT_EnableVoiceTransmission(self.ptr.0, if enable { 1 } else { 0 }) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Firewall_AddAppException`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2249:<li><a class="el" href="group__firewall.html#gabd484e8dc9124a8e87850c91a79f7a92" title="Add an application to the Windows Firewall exception list.">TT_Firewall_AddAppException()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.js:6:    [ "TT_Firewall_AddAppException", "group__firewall.html#gabd484e8dc9124a8e87850c91a79f7a92", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:106:<tr class="memitem:gabd484e8dc9124a8e87850c91a79f7a92"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__firewall.html#gabd484e8dc9124a8e87850c91a79f7a92">TT_Firewall_AddAppException</a> (IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szName, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szExecutable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:117:<p>Check out <a class="el" href="group__firewall.html#gabd484e8dc9124a8e87850c91a79f7a92" title="Add an application to the Windows Firewall exception list.">TT_Firewall_AddAppException()</a> on how to add application executables to the Windows Firewall exception list. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:179:<p>This function does not invoke UAC on Windows Vista/7. </p><dl class="section see"><dt>See also</dt><dd><a class="el" href="group__firewall.html#gabd484e8dc9124a8e87850c91a79f7a92" title="Add an application to the Windows Firewall exception list.">TT_Firewall_AddAppException</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:184:<h2 class="memtitle"><span class="permalink"><a href="#gabd484e8dc9124a8e87850c91a79f7a92">&#9670;&nbsp;</a></span>TT_Firewall_AddAppException()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:190:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Firewall_AddAppException </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:235:<a class="el" href="group__firewall.html#gabd484e8dc9124a8e87850c91a79f7a92" title="Add an application to the Windows Firewall exception list.">TT_Firewall_AddAppException</a> </dd></dl>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:3015:    TEAMTALKDLL_API TTBOOL TT_Firewall_AddAppException(IN const TTCHAR* szName, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:47:            ffi::api().TT_Firewall_AddAppException(name.tt().as_ptr(), exe_path.tt().as_ptr()) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Firewall_AppExceptionExists`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2248:<li><a class="el" href="group__firewall.html#gafc3c4b8c13beee6e4db87d3e77aba2eb" title="Check if an executable is already in the Windows Firewall exception list.">TT_Firewall_AppExceptionExists()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:286:<li>TT_Firewall_AppExceptionExists()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:340:<li>TT_Firewall_AppExceptionExists()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.js:5:    [ "TT_Firewall_AppExceptionExists", "group__firewall.html#gafc3c4b8c13beee6e4db87d3e77aba2eb", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:103:<tr class="memitem:gafc3c4b8c13beee6e4db87d3e77aba2eb"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__firewall.html#gafc3c4b8c13beee6e4db87d3e77aba2eb">TT_Firewall_AppExceptionExists</a> (IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szExecutable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:163:<h2 class="memtitle"><span class="permalink"><a href="#gafc3c4b8c13beee6e4db87d3e77aba2eb">&#9670;&nbsp;</a></span>TT_Firewall_AppExceptionExists()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:169:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Firewall_AppExceptionExists </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:210:<p>On Windows XP (SP2+) the user calling this function is assumed to have administrator rights. On Windows Vista/7 UAC is invoked to ask the user for administrator rights. </p><dl class="section see"><dt>See also</dt><dd><a class="el" href="group__firewall.html#gafc3c4b8c13beee6e4db87d3e77aba2eb" title="Check if an executable is already in the Windows Firewall exception list.">TT_Firewall_AppExceptionExists</a> </dd>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:3012:    TEAMTALKDLL_API TTBOOL TT_Firewall_AppExceptionExists(IN const TTCHAR* szExecutable);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:34:        unsafe { ffi::api().TT_Firewall_AppExceptionExists(exe_path.tt().as_ptr()) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Firewall_Enable`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2247:<li><a class="el" href="group__firewall.html#gabf65e8ce96e138e056c597f0d48ca5a9" title="Enable/disable the Windows Firewall.">TT_Firewall_Enable()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.js:4:    [ "TT_Firewall_Enable", "group__firewall.html#gabf65e8ce96e138e056c597f0d48ca5a9", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:289:<li>TT_Firewall_Enable()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:343:<li>TT_Firewall_Enable()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:100:<tr class="memitem:gabf65e8ce96e138e056c597f0d48ca5a9"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__firewall.html#gabf65e8ce96e138e056c597f0d48ca5a9">TT_Firewall_Enable</a> (IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEnable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:136:<p>This function does not invoke UAC on Windows Vista/7. </p><dl class="section see"><dt>See also</dt><dd><a class="el" href="group__firewall.html#gabf65e8ce96e138e056c597f0d48ca5a9" title="Enable/disable the Windows Firewall.">TT_Firewall_Enable</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:141:<h2 class="memtitle"><span class="permalink"><a href="#gabf65e8ce96e138e056c597f0d48ca5a9">&#9670;&nbsp;</a></span>TT_Firewall_Enable()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:147:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Firewall_Enable </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:3009:    TEAMTALKDLL_API TTBOOL TT_Firewall_Enable(IN TTBOOL bEnable);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:40:        unsafe { ffi::api().TT_Firewall_Enable(if enable { 1 } else { 0 }) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Firewall_IsEnabled`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:292:<li>TT_Firewall_IsEnabled()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:346:<li>TT_Firewall_IsEnabled()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.js:3:    [ "TT_Firewall_IsEnabled", "group__firewall.html#ga3fc7292904c82a6b3ef5d4a752cba5a0", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:97:<tr class="memitem:ga3fc7292904c82a6b3ef5d4a752cba5a0"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__firewall.html#ga3fc7292904c82a6b3ef5d4a752cba5a0">TT_Firewall_IsEnabled</a> (void)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:120:<h2 class="memtitle"><span class="permalink"><a href="#ga3fc7292904c82a6b3ef5d4a752cba5a0">&#9670;&nbsp;</a></span>TT_Firewall_IsEnabled()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:126:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Firewall_IsEnabled </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:158:<p>On Windows XP (SP2+) the user calling this function is assumed to have administrator rights. On Windows Vista/7 UAC is invoked to ask the user for administrator rights. </p><dl class="section see"><dt>See also</dt><dd><a class="el" href="group__firewall.html#ga3fc7292904c82a6b3ef5d4a752cba5a0" title="Check if the Windows Firewall is currently enabled.">TT_Firewall_IsEnabled</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:98:  ['tt_5ffirewall_5fisenabled_762',['TT_Firewall_IsEnabled',['../group__firewall.html#ga3fc7292904c82a6b3ef5d4a752cba5a0',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:3006:    TEAMTALKDLL_API TTBOOL TT_Firewall_IsEnabled(void);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:28:        unsafe { ffi::api().TT_Firewall_IsEnabled() == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Firewall_RemoveAppException`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2250:<li><a class="el" href="group__firewall.html#ga8513258dad87a033d6bdf46509f391cc" title="Remove an application from the Windows Firewall exception list.">TT_Firewall_RemoveAppException()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:295:<li>TT_Firewall_RemoveAppException()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:349:<li>TT_Firewall_RemoveAppException()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:109:<tr class="memitem:ga8513258dad87a033d6bdf46509f391cc"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__firewall.html#ga8513258dad87a033d6bdf46509f391cc">TT_Firewall_RemoveAppException</a> (IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szExecutable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:212:<a class="el" href="group__firewall.html#ga8513258dad87a033d6bdf46509f391cc" title="Remove an application from the Windows Firewall exception list.">TT_Firewall_RemoveAppException</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:217:<h2 class="memtitle"><span class="permalink"><a href="#ga8513258dad87a033d6bdf46509f391cc">&#9670;&nbsp;</a></span>TT_Firewall_RemoveAppException()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.html:223:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Firewall_RemoveAppException </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__firewall.js:7:    [ "TT_Firewall_RemoveAppException", "group__firewall.html#ga8513258dad87a033d6bdf46509f391cc", null ]
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:3019:    TEAMTALKDLL_API TTBOOL TT_Firewall_RemoveAppException(IN const TTCHAR* szExecutable);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:54:        unsafe { ffi::api().TT_Firewall_RemoveAppException(exe_path.tt().as_ptr()) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1116:<p>In TeamTalk 4 events were posted in a <code><a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a></code> containing a WPARAM and LPARAM where the WPARAM would typically contain an ID and the LPARAM some extended information about the event. When an event occured the client application would query the TeamTalk client instance to extract information about what had changed to the current state. Since the client instance was running in its own thread the state change could, however, have become unknown in the meantime. If e.g. a text message was received from a user and the user would immediately quit afterwards then the text message would be lost since the text message was "attached" to the user who was no longer there. In TeamTalk 5 a <a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a> now contains a copy of what changed as a cause of the event. If e.g. a text message is received from a user then the <a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a> will contain a full copy of the <a class="el" href="struct_text_message.html" title="A struct containing the properties of a text message sent by a user.">TextMessage</a> which was received. Also if a new channel is created then it's no longer required to extract the <a class="el" href="struct_channel.html" title="A struct containing the properties of a channel.">Channel</a> object through the TeamTalk client instance (using <a class="el" href="group__channels.html#gabdca08af83e08dd77bcd62077f30b638" title="Get the channel with a specific ID.">TT_GetChannel()</a>) now the new channel is part of the <a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a> using the <a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a>'s data container.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1686:<li><a class="el" href="group__channels.html#ga7a0961d699bb88a215974852823af24f" title="Get information about a file which can be downloaded.">TT_GetChannelFile()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1687:<li>Replacement for <code>TT_GetChannelFileInfo</code> </li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1750:<li><a class="el" href="group__channels.html#gacb7b939aca021bf80fc32a02f89ba508" title="Get the IDs of all users in a channel.">TT_GetChannelUsers()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1754:<li><a class="el" href="group__channels.html#gac6975b0b86563b9b5eae7bd9646d50dd" title="Get the list of the files in a channel which can be downloaded.">TT_GetChannelFiles()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1898:<li><code>TT_GetChannelFileInfo()</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1899:<li>Use <a class="el" href="group__channels.html#ga7a0961d699bb88a215974852823af24f" title="Get information about a file which can be downloaded.">TT_GetChannelFile()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2569:<li><code>TT_GetChannelParentID</code> <ul>
... (100 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2667:    TEAMTALKDLL_API TTBOOL TT_GetChannel(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2672:    TEAMTALKDLL_API TTBOOL TT_GetChannelPath(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2677:    TEAMTALKDLL_API INT32 TT_GetChannelIDFromPath(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2681:    TEAMTALKDLL_API TTBOOL TT_GetChannelUsers(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2687:    TEAMTALKDLL_API TTBOOL TT_GetChannelFiles(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2693:    TEAMTALKDLL_API TTBOOL TT_GetChannelFile(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:289:        if unsafe { ffi::api().TT_GetChannel(ptr, channel_id, &mut raw) } == 1 {
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:52:            if ffi::api().TT_GetChannelPath(self.ptr.0, id.0, buf.as_mut_ptr()) == 1 {
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:62:        ChannelId(unsafe { ffi::api().TT_GetChannelIDFromPath(self.ptr.0, path.tt().as_ptr()) })
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:181:            ffi::api().TT_GetChannelUsers(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:188:            if ffi::api().TT_GetChannelUsers(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:21:            ffi::api().TT_GetChannelFiles(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:28:            if ffi::api().TT_GetChannelFiles(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetChannelFile`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1686:<li><a class="el" href="group__channels.html#ga7a0961d699bb88a215974852823af24f" title="Get information about a file which can be downloaded.">TT_GetChannelFile()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1687:<li>Replacement for <code>TT_GetChannelFileInfo</code> </li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1754:<li><a class="el" href="group__channels.html#gac6975b0b86563b9b5eae7bd9646d50dd" title="Get the list of the files in a channel which can be downloaded.">TT_GetChannelFiles()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1898:<li><code>TT_GetChannelFileInfo()</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1899:<li>Use <a class="el" href="group__channels.html#ga7a0961d699bb88a215974852823af24f" title="Get information about a file which can be downloaded.">TT_GetChannelFile()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2633:<li><code>TT_GetChannelFilesCount</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2634:<li>Call <a class="el" href="group__channels.html#gac6975b0b86563b9b5eae7bd9646d50dd" title="Get the list of the files in a channel which can be downloaded.">TT_GetChannelFiles</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2637:<li><code>TT_GetChannelFileID</code> <ul>
... (32 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2687:    TEAMTALKDLL_API TTBOOL TT_GetChannelFiles(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2693:    TEAMTALKDLL_API TTBOOL TT_GetChannelFile(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:21:            ffi::api().TT_GetChannelFiles(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:28:            if ffi::api().TT_GetChannelFiles(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetChannelFiles`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1754:<li><a class="el" href="group__channels.html#gac6975b0b86563b9b5eae7bd9646d50dd" title="Get the list of the files in a channel which can be downloaded.">TT_GetChannelFiles()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2633:<li><code>TT_GetChannelFilesCount</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2634:<li>Call <a class="el" href="group__channels.html#gac6975b0b86563b9b5eae7bd9646d50dd" title="Get the list of the files in a channel which can be downloaded.">TT_GetChannelFiles</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2638:<li>Call <a class="el" href="group__channels.html#gac6975b0b86563b9b5eae7bd9646d50dd" title="Get the list of the files in a channel which can be downloaded.">TT_GetChannelFiles</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:304:<li>TT_GetChannelFiles()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:197:<tr class="memitem:gac6975b0b86563b9b5eae7bd9646d50dd"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#gac6975b0b86563b9b5eae7bd9646d50dd">TT_GetChannelFiles</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID, IN OUT <a class="el" href="struct_remote_file.html">RemoteFile</a> *lpRemoteFiles, IN OUT INT32 *lpnHowMany)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:816:<h2 class="memtitle"><span class="permalink"><a href="#gac6975b0b86563b9b5eae7bd9646d50dd">&#9670;&nbsp;</a></span>TT_GetChannelFiles()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:822:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetChannelFiles </td>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2687:    TEAMTALKDLL_API TTBOOL TT_GetChannelFiles(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:21:            ffi::api().TT_GetChannelFiles(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:28:            if ffi::api().TT_GetChannelFiles(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetChannelIDFromPath`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:307:<li>TT_GetChannelIDFromPath()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:361:<li>TT_GetChannelIDFromPath()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:76:    [ "TT_GetChannelIDFromPath", "group__channels.html#gabdbd4d031658162720d4c195b35ec016", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:191:<tr class="memitem:gabdbd4d031658162720d4c195b35ec016"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#gabdbd4d031658162720d4c195b35ec016">TT_GetChannelIDFromPath</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szChannelPath)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:724:<h2 class="memtitle"><span class="permalink"><a href="#gabdbd4d031658162720d4c195b35ec016">&#9670;&nbsp;</a></span>TT_GetChannelIDFromPath()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:730:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_GetChannelIDFromPath </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:103:  ['tt_5fgetchannelidfrompath_767',['TT_GetChannelIDFromPath',['../group__channels.html#gabdbd4d031658162720d4c195b35ec016',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:1783:<div class="line"><a name="l07179"></a><span class="lineno"><a class="line" href="group__channels.html#gabdbd4d031658162720d4c195b35ec016"> 7179</a></span>&#160;    <a class="code" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 <a class="code" href="group__channels.html#gabdbd4d031658162720d4c195b35ec016">TT_GetChannelIDFromPath</a>(IN <a class="code" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a>* lpTTInstance,</div>
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2677:    TEAMTALKDLL_API INT32 TT_GetChannelIDFromPath(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:62:        ChannelId(unsafe { ffi::api().TT_GetChannelIDFromPath(self.ptr.0, path.tt().as_ptr()) })
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetChannelPath`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:310:<li>TT_GetChannelPath()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:364:<li>TT_GetChannelPath()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:75:    [ "TT_GetChannelPath", "group__channels.html#ga689ef5a8bebc730d01ef7dad1f60e0f8", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:188:<tr class="memitem:ga689ef5a8bebc730d01ef7dad1f60e0f8"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#ga689ef5a8bebc730d01ef7dad1f60e0f8">TT_GetChannelPath</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID, OUT <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szChannelPath[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:602:<a class="el" href="group__channels.html#ga689ef5a8bebc730d01ef7dad1f60e0f8" title="Get the channel&#39;s path. Channels are separated by &#39;/&#39;.">TT_GetChannelPath</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:679:<h2 class="memtitle"><span class="permalink"><a href="#ga689ef5a8bebc730d01ef7dad1f60e0f8">&#9670;&nbsp;</a></span>TT_GetChannelPath()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:685:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetChannelPath </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:104:  ['tt_5fgetchannelpath_768',['TT_GetChannelPath',['../group__channels.html#ga689ef5a8bebc730d01ef7dad1f60e0f8',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2672:    TEAMTALKDLL_API TTBOOL TT_GetChannelPath(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:52:            if ffi::api().TT_GetChannelPath(self.ptr.0, id.0, buf.as_mut_ptr()) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetChannelUsers`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1750:<li><a class="el" href="group__channels.html#gacb7b939aca021bf80fc32a02f89ba508" title="Get the IDs of all users in a channel.">TT_GetChannelUsers()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:194:<tr class="memitem:gacb7b939aca021bf80fc32a02f89ba508"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#gacb7b939aca021bf80fc32a02f89ba508">TT_GetChannelUsers</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID, IN OUT <a class="el" href="struct_user.html">User</a> *lpUsers, IN OUT INT32 *lpnHowMany)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:762:<h2 class="memtitle"><span class="permalink"><a href="#gacb7b939aca021bf80fc32a02f89ba508">&#9670;&nbsp;</a></span>TT_GetChannelUsers()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:768:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetChannelUsers </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:367:<li>TT_GetChannelUsers()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:313:<li>TT_GetChannelUsers()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:77:    [ "TT_GetChannelUsers", "group__channels.html#gacb7b939aca021bf80fc32a02f89ba508", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:752:<p>If only users in a specific channel is needed call <a class="el" href="group__channels.html#gacb7b939aca021bf80fc32a02f89ba508" title="Get the IDs of all users in a channel.">TT_GetChannelUsers()</a></p>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2681:    TEAMTALKDLL_API TTBOOL TT_GetChannelUsers(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:181:            ffi::api().TT_GetChannelUsers(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:188:            if ffi::api().TT_GetChannelUsers(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetClientKeepAlive`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:771:<p>Now it's possible to configure the keep alive properties using <a class="el" href="struct_client_keep_alive.html" title="Control timers for sending keep alive information to the server.">ClientKeepAlive</a>-struct and functions <a class="el" href="group__connectivity.html#gadf4cc840006b7c4f49caac2f63ad3e5f" title="Update the client instance&#39;s default keep alive settings.">TT_SetClientKeepAlive()</a> and <a class="el" href="group__connectivity.html#ga5dacbde76801d119b1045a87f4fa7c25" title="Get the client instance&#39;s current keep alive settings.">TT_GetClientKeepAlive()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:797:<li>New functions <a class="el" href="group__connectivity.html#gadf4cc840006b7c4f49caac2f63ad3e5f" title="Update the client instance&#39;s default keep alive settings.">TT_SetClientKeepAlive()</a> and <a class="el" href="group__connectivity.html#ga5dacbde76801d119b1045a87f4fa7c25" title="Get the client instance&#39;s current keep alive settings.">TT_GetClientKeepAlive()</a> for keep alive properties.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:370:<li>TT_GetClientKeepAlive()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:316:<li>TT_GetClientKeepAlive()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:153:<tr class="memitem:ga5dacbde76801d119b1045a87f4fa7c25"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__connectivity.html#ga5dacbde76801d119b1045a87f4fa7c25">TT_GetClientKeepAlive</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, OUT <a class="el" href="struct_client_keep_alive.html">ClientKeepAlive</a> *lpClientKeepAlive)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:647:<a class="el" href="group__connectivity.html#ga5dacbde76801d119b1045a87f4fa7c25" title="Get the client instance&#39;s current keep alive settings.">TT_GetClientKeepAlive()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:654:<h2 class="memtitle"><span class="permalink"><a href="#ga5dacbde76801d119b1045a87f4fa7c25">&#9670;&nbsp;</a></span>TT_GetClientKeepAlive()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:660:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetClientKeepAlive </td>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2473:    TEAMTALKDLL_API TTBOOL TT_GetClientKeepAlive(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\connection.rs:611:        if unsafe { ffi::api().TT_GetClientKeepAlive(self.ptr.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetClientStatistics`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1678:<li><a class="el" href="group__connectivity.html#gad22d782d263f9505239932285fc99e80" title="Retrieve client statistics of bandwidth usage and response times.">TT_GetClientStatistics()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1890:<li>Use <a class="el" href="group__connectivity.html#gad22d782d263f9505239932285fc99e80" title="Retrieve client statistics of bandwidth usage and response times.">TT_GetClientStatistics()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:319:<li>TT_GetClientStatistics()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:373:<li>TT_GetClientStatistics()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.js:55:    [ "TT_GetClientStatistics", "group__connectivity.html#gad22d782d263f9505239932285fc99e80", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:147:<tr class="memitem:gad22d782d263f9505239932285fc99e80"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__connectivity.html#gad22d782d263f9505239932285fc99e80">TT_GetClientStatistics</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, OUT <a class="el" href="struct_client_statistics.html">ClientStatistics</a> *lpClientStatistics)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:215:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__connectivity.html#gad22d782d263f9505239932285fc99e80" title="Retrieve client statistics of bandwidth usage and response times.">TT_GetClientStatistics</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:586:<h2 class="memtitle"><span class="permalink"><a href="#gad22d782d263f9505239932285fc99e80">&#9670;&nbsp;</a></span>TT_GetClientStatistics()</h2>
... (11 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2465:     TEAMTALKDLL_API TTBOOL TT_GetClientStatistics(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:52:        if unsafe { ffi::api().TT_GetClientStatistics(self.ptr.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetDefaultSoundDevices`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1710:<li><a class="el" href="group__sounddevices.html#ga2d4e2143314d30b15411dda958580bef" title="Get the default sound devices.">TT_GetDefaultSoundDevices()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2067:<li><a class="el" href="group__sounddevices.html#ga388b57b9d0dad39b8a3ce61a22a35090" title="Get the default sound devices for the specified sound system.">TT_GetDefaultSoundDevicesEx()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2454:<li>Call <a class="el" href="group__sounddevices.html#ga2d4e2143314d30b15411dda958580bef" title="Get the default sound devices.">TT_GetDefaultSoundDevices</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:322:<li>TT_GetDefaultSoundDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:325:<li>TT_GetDefaultSoundDevicesEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:376:<li>TT_GetDefaultSoundDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:379:<li>TT_GetDefaultSoundDevicesEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:222:<tr class="memitem:ga2d4e2143314d30b15411dda958580bef"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga2d4e2143314d30b15411dda958580bef">TT_GetDefaultSoundDevices</a> (OUT INT32 *lpnInputDeviceID, OUT INT32 *lpnOutputDeviceID)</td></tr>
... (24 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1999:    TEAMTALKDLL_API TTBOOL TT_GetDefaultSoundDevices(OUT INT32* lpnInputDeviceID, 
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2002:    TEAMTALKDLL_API TTBOOL TT_GetDefaultSoundDevicesEx(IN SoundSystem nSndSystem, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:57:            ffi::api().TT_GetDefaultSoundDevices(&mut input, &mut output);
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:67:            ffi::api().TT_GetDefaultSoundDevicesEx(system, &mut input, &mut output);
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetDefaultSoundDevicesEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2067:<li><a class="el" href="group__sounddevices.html#ga388b57b9d0dad39b8a3ce61a22a35090" title="Get the default sound devices for the specified sound system.">TT_GetDefaultSoundDevicesEx()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:379:<li>TT_GetDefaultSoundDevicesEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:325:<li>TT_GetDefaultSoundDevicesEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:82:    [ "TT_GetDefaultSoundDevicesEx", "group__sounddevices.html#ga388b57b9d0dad39b8a3ce61a22a35090", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:225:<tr class="memitem:ga388b57b9d0dad39b8a3ce61a22a35090"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga388b57b9d0dad39b8a3ce61a22a35090">TT_GetDefaultSoundDevicesEx</a> (IN <a class="el" href="group__sounddevices.html#ga2290e667ea32ff3734fb88796abfd267">SoundSystem</a> nSndSystem, OUT INT32 *lpnInputDeviceID, OUT INT32 *lpnOutputDeviceID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:950:<h2 class="memtitle"><span class="permalink"><a href="#ga388b57b9d0dad39b8a3ce61a22a35090">&#9670;&nbsp;</a></span>TT_GetDefaultSoundDevicesEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:956:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetDefaultSoundDevicesEx </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:109:  ['tt_5fgetdefaultsounddevicesex_773',['TT_GetDefaultSoundDevicesEx',['../group__sounddevices.html#ga388b57b9d0dad39b8a3ce61a22a35090',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2002:    TEAMTALKDLL_API TTBOOL TT_GetDefaultSoundDevicesEx(IN SoundSystem nSndSystem, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:67:            ffi::api().TT_GetDefaultSoundDevicesEx(system, &mut input, &mut output);
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetErrorMessage`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:328:<li>TT_GetErrorMessage()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:382:<li>TT_GetErrorMessage()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:179:<tr class="memitem:ga062a47484a181bc180f5a9ab76104d41"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> void&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__errorhandling.html#ga062a47484a181bc180f5a9ab76104d41">TT_GetErrorMessage</a> (IN INT32 nError, OUT <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szErrorMsg[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:201:<p>Use <a class="el" href="group__errorhandling.html#ga062a47484a181bc180f5a9ab76104d41" title="Get textual discription of an error message.">TT_GetErrorMessage</a> to get a text-description of the error. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:235:<p>Use <a class="el" href="group__errorhandling.html#ga062a47484a181bc180f5a9ab76104d41" title="Get textual discription of an error message.">TT_GetErrorMessage</a> to get a text-description of the error. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:476:<h2 class="memtitle"><span class="permalink"><a href="#ga062a47484a181bc180f5a9ab76104d41">&#9670;&nbsp;</a></span>TT_GetErrorMessage()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:482:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> void TT_GetErrorMessage </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.js:56:    [ "TT_GetErrorMessage", "group__errorhandling.html#ga062a47484a181bc180f5a9ab76104d41", null ]
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2830:    TEAMTALKDLL_API void TT_GetErrorMessage(IN INT32 nError, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:1010:            ffi::api().TT_GetErrorMessage(code, buf.as_mut_ptr());
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetFileTransferInfo`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:385:<li>TT_GetFileTransferInfo()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:82:    [ "TT_GetFileTransferInfo", "group__channels.html#ga263fe2dfca9a756ecf750492bb4f37f3", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:686:<p>Use <a class="el" href="group__channels.html#ga263fe2dfca9a756ecf750492bb4f37f3" title="Get information about an active file transfer.">TT_GetFileTransferInfo</a> to get information about the file transfer. Ensure to check if the file transfer is completed, because the file transfer instance will be removed from the client instance when the user application reads the <a class="el" href="struct_file_transfer.html" title="A struct containing the properties of a file transfer.">FileTransfer</a> object and it has completed the transfer.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:692:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__channels.html#ga263fe2dfca9a756ecf750492bb4f37f3" title="Get information about an active file transfer.">TT_GetFileTransferInfo</a> To retrieve <a class="el" href="struct_file_transfer.html" title="A struct containing the properties of a file transfer.">FileTransfer</a>. </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:331:<li>TT_GetFileTransferInfo()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:209:<tr class="memitem:ga263fe2dfca9a756ecf750492bb4f37f3"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#ga263fe2dfca9a756ecf750492bb4f37f3">TT_GetFileTransferInfo</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nTransferID, OUT <a class="el" href="struct_file_transfer.html">FileTransfer</a> *lpFileTransfer)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:482:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__channels.html#ga263fe2dfca9a756ecf750492bb4f37f3" title="Get information about an active file transfer.">TT_GetFileTransferInfo</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:1000:<h2 class="memtitle"><span class="permalink"><a href="#ga263fe2dfca9a756ecf750492bb4f37f3">&#9670;&nbsp;</a></span>TT_GetFileTransferInfo()</h2>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2821:    TEAMTALKDLL_API TTBOOL TT_GetFileTransferInfo(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\files.rs:79:        if unsafe { ffi::api().TT_GetFileTransferInfo(self.ptr.0, transfer_id.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetFlags`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1843:<li>Use <a class="el" href="group__initclient.html#ga80a8e7d232eb05c3733a3bc9edf461dd" title="Get a bitmask describing the client&#39;s current state.">TT_GetFlags()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:334:<li>TT_GetFlags()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:388:<li>TT_GetFlags()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:165:<tr class="memitem:ga80a8e7d232eb05c3733a3bc9edf461dd"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="group__initclient.html#ga45632f9da11731b15d9bbb90713764b0">ClientFlags</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__initclient.html#ga80a8e7d232eb05c3733a3bc9edf461dd">TT_GetFlags</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:175:<p>When a new client instance is created a user application can call to <a class="el" href="group__initclient.html#ga80a8e7d232eb05c3733a3bc9edf461dd" title="Get a bitmask describing the client&#39;s current state.">TT_GetFlags</a> to query the client instance's current state. Initially the client instance's state will be <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1ca74a7ca0a423000407d515e604f1db5d0" title="The client instance (TTInstance) is in closed state, i.e. TT_InitTeamTalk has return a valid instance...">CLIENT_CLOSED</a>. This means that no operation has been performed on the client. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:191:<p>The state of the client instance can be retrieved by calling <a class="el" href="group__initclient.html#ga80a8e7d232eb05c3733a3bc9edf461dd" title="Get a bitmask describing the client&#39;s current state.">TT_GetFlags</a>. This enables the user application to display the possible options to the end user. If e.g. the flag <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1caa1b074d1e989175f0d96041df68478ee" title="If set the client instance is logged on to a server, i.e. got CLIENTEVENT_CMD_MYSELF_LOGGEDIN event a...">CLIENT_AUTHORIZED</a> is not set it will not be possible to perform any other commands except <a class="el" href="group__commands.html#ga9d15454938054ddb66ebe16f88e2efaa" title="Same as TT_DologinEx() but without the option to specify szClientName. Kept for backwards compatibili...">TT_DoLogin</a>. Doing so will make the server return an error message to the client. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:248:<p>The state of the client instance can be retrieved by calling <a class="el" href="group__initclient.html#ga80a8e7d232eb05c3733a3bc9edf461dd" title="Get a bitmask describing the client&#39;s current state.">TT_GetFlags</a>. This enables the user application to display the possible options to the end user. If e.g. the flag <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1caa1b074d1e989175f0d96041df68478ee" title="If set the client instance is logged on to a server, i.e. got CLIENTEVENT_CMD_MYSELF_LOGGEDIN event a...">CLIENT_AUTHORIZED</a> is not set it will not be possible to perform any other commands except <a class="el" href="group__commands.html#ga9d15454938054ddb66ebe16f88e2efaa" title="Same as TT_DologinEx() but without the option to specify szClientName. Kept for backwards compatibili...">TT_DoLogin</a>. Doing so will make the server return an error message to the client. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:557:<h2 class="memtitle"><span class="permalink"><a href="#ga80a8e7d232eb05c3733a3bc9edf461dd">&#9670;&nbsp;</a></span>TT_GetFlags()</h2>
... (14 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1989:     TEAMTALKDLL_API ClientFlags TT_GetFlags(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:380:        unsafe { ffi::api().TT_GetFlags(ptr) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMediaFileInfo`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2019:<li><a class="el" href="group__mediastream.html#ga7dd4abb54a231a55d02f877b9b1986ce" title="Get the properties of a media file.">TT_GetMediaFileInfo()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:391:<li>TT_GetMediaFileInfo()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:337:<li>TT_GetMediaFileInfo()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.js:60:    [ "TT_GetMediaFileInfo", "group__mediastream.html#ga7dd4abb54a231a55d02f877b9b1986ce", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:193:<tr class="memitem:ga7dd4abb54a231a55d02f877b9b1986ce"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#ga7dd4abb54a231a55d02f877b9b1986ce">TT_GetMediaFileInfo</a> (IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szMediaFilePath, OUT <a class="el" href="struct_media_file_info.html">MediaFileInfo</a> *lpMediaFileInfo)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:207:<p>Use <a class="el" href="group__mediastream.html#ga7dd4abb54a231a55d02f877b9b1986ce" title="Get the properties of a media file.">TT_GetMediaFileInfo()</a> to see if the media file format is supported and what properties are used for audio and video.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:276:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mediastream.html#ga7dd4abb54a231a55d02f877b9b1986ce" title="Get the properties of a media file.">TT_GetMediaFileInfo()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:494:<p>Call <a class="el" href="group__mediastream.html#ga7dd4abb54a231a55d02f877b9b1986ce" title="Get the properties of a media file.">TT_GetMediaFileInfo()</a> to get the properties of a media file, i.e. audio and video format.</p>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2275:    TEAMTALKDLL_API TTBOOL TT_GetMediaFileInfo(IN const TTCHAR* szMediaFilePath,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMessage`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2561:<li><code>TT_GetMessageOfTheDay</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\examples.html:278:<p>This is a simple console application which stores all audio sent to the server to a specific directory on disk. It also displays all user text chat sessions, file uploads, etc. This example gives a good idea of how events are processed in TeamTalk when using <a class="el" href="group__initclient.html#gaebc89ca414258f4e8228f8af91343e72" title="Create a new TeamTalk client instance where events are &#39;polled&#39; using TT_GetMessage.">TT_InitTeamTalkPoll()</a> and events are not posted to a window handle.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:253:<p>When events occur in the client instance, like e.g. if a new user joins a channel, the client instance queues a <a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a> which the user application must retrieve using <a class="el" href="group__initclient.html#ga34fe8de6133a101aa70574225d7dcae0" title="Poll for events in the client instance.">TT_GetMessage()</a>. The message queue for events is limited to 1 MB. If the queue grows above the maximum size then event handling is suspended and will not be resumed until the queue size is again below the maximum size. The event <a class="el" href="group__errorhandling.html#ggafc4bdfbf2ff7f70d54e072c3fe3f2c6cab3b176398dbf066871f502f0a81082ce" title="TTMessage event queue overflowed.">INTERR_TTMESSAGE_QUEUE_OVERFLOW</a> will be posted to the message queue if an overflow has taken place.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:271:<p>Events are retrieved using <a class="el" href="group__initclient.html#ga34fe8de6133a101aa70574225d7dcae0" title="Poll for events in the client instance.">TT_GetMessage()</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:302:<p>The event can be retrieved by called <a class="el" href="group__initclient.html#ga34fe8de6133a101aa70574225d7dcae0" title="Poll for events in the client instance.">TT_GetMessage</a>. This struct is only required on non-Windows systems.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:304:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__initclient.html#ga34fe8de6133a101aa70574225d7dcae0" title="Poll for events in the client instance.">TT_GetMessage</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:322:<p>Events are retrieved using <a class="el" href="group__initclient.html#ga34fe8de6133a101aa70574225d7dcae0" title="Poll for events in the client instance.">TT_GetMessage()</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:394:<li>TT_GetMessage()
... (27 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1979:    TEAMTALKDLL_API TTBOOL TT_GetMessage(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:809:        if unsafe { ffi::api().TT_GetMessage(self.ptr.0, &mut msg, &t) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMyChannelID`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:343:<li>TT_GetMyChannelID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:397:<li>TT_GetMyChannelID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:182:<tr class="memitem:ga8af373c178ae8285c47edfe66fd3da22"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#ga8af373c178ae8285c47edfe66fd3da22">TT_GetMyChannelID</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:600:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__channels.html#ga8af373c178ae8285c47edfe66fd3da22" title="Get the channel which the local client instance is currently participating in.">TT_GetMyChannelID</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:607:<h2 class="memtitle"><span class="permalink"><a href="#ga8af373c178ae8285c47edfe66fd3da22">&#9670;&nbsp;</a></span>TT_GetMyChannelID()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:613:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_GetMyChannelID </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:73:    [ "TT_GetMyChannelID", "group__channels.html#ga8af373c178ae8285c47edfe66fd3da22", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:588:    <tr><td class="paramname">lpAudioCodec</td><td>The audio codec which should be used as reference for muxing users' audio streams. In most situations this is the <a class="el" href="struct_audio_codec.html" title="Struct used for specifying which audio codec a channel uses.">AudioCodec</a> of the current channel, i.e. <a class="el" href="group__channels.html#ga8af373c178ae8285c47edfe66fd3da22" title="Get the channel which the local client instance is currently participating in.">TT_GetMyChannelID()</a>. </td></tr>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2664:    TEAMTALKDLL_API INT32 TT_GetMyChannelID(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:305:        ChannelId(unsafe { ffi::api().TT_GetMyChannelID(ptr) })
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMyUserAccount`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1690:<li><a class="el" href="group__users.html#ga8d8a0be51387862fa4e58004b4a388a2" title="Convenience method for TT_GetMyUserAccount()">TT_GetMyUserRights()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2082:<p>A <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> now has the <code>szInitChannel</code> property which holds the channel a user should join after login. The user can join this channel without passing a password. Use <a class="el" href="group__users.html#ga9ae176938d27d34bd719dc3df89407d7" title="Get the local client instance&#39;s UserAccount.">TT_GetMyUserAccount()</a> to get the local instance's <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> after login has completed.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2382:<p>After login it's now possible to retrieve one's own <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> by calling <a class="el" href="group__users.html#ga9ae176938d27d34bd719dc3df89407d7" title="Get the local client instance&#39;s UserAccount.">TT_GetMyUserAccount()</a>. <a class="el" href="group__users.html#gae8dd6c7fbdeead08735e9ce83e16d1b2" title="If an account was used in TT_DoLogin then this value will return the nUserData from the UserAccount.">TT_GetMyUserData()</a> can be used to extract one's <em>nUserData</em> of one's <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:346:<li>TT_GetMyUserAccount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:400:<li>TT_GetMyUserAccount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:210:<tr class="memitem:ga9ae176938d27d34bd719dc3df89407d7"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__users.html#ga9ae176938d27d34bd719dc3df89407d7">TT_GetMyUserAccount</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, OUT <a class="el" href="struct_user_account.html">UserAccount</a> *lpUserAccount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:217:<tr class="memdesc:ga8d8a0be51387862fa4e58004b4a388a2"><td class="mdescLeft">&#160;</td><td class="mdescRight">Convenience method for <a class="el" href="group__users.html#ga9ae176938d27d34bd719dc3df89407d7" title="Get the local client instance&#39;s UserAccount.">TT_GetMyUserAccount()</a>  <a href="group__users.html#ga8d8a0be51387862fa4e58004b4a388a2">More...</a><br /></td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:659:<h2 class="memtitle"><span class="permalink"><a href="#ga9ae176938d27d34bd719dc3df89407d7">&#9670;&nbsp;</a></span>TT_GetMyUserAccount()</h2>
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2715:    TEAMTALKDLL_API TTBOOL TT_GetMyUserAccount(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:260:        if unsafe { ffi::api().TT_GetMyUserAccount(self.ptr.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMyUserData`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2382:<p>After login it's now possible to retrieve one's own <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> by calling <a class="el" href="group__users.html#ga9ae176938d27d34bd719dc3df89407d7" title="Get the local client instance&#39;s UserAccount.">TT_GetMyUserAccount()</a>. <a class="el" href="group__users.html#gae8dd6c7fbdeead08735e9ce83e16d1b2" title="If an account was used in TT_DoLogin then this value will return the nUserData from the UserAccount.">TT_GetMyUserData()</a> can be used to extract one's <em>nUserData</em> of one's <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:349:<li>TT_GetMyUserData()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:403:<li>TT_GetMyUserData()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.js:110:    [ "TT_GetMyUserData", "group__users.html#gae8dd6c7fbdeead08735e9ce83e16d1b2", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:219:<tr class="memitem:gae8dd6c7fbdeead08735e9ce83e16d1b2"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__users.html#gae8dd6c7fbdeead08735e9ce83e16d1b2">TT_GetMyUserData</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:757:<h2 class="memtitle"><span class="permalink"><a href="#gae8dd6c7fbdeead08735e9ce83e16d1b2">&#9670;&nbsp;</a></span>TT_GetMyUserData()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:763:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_GetMyUserData </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:117:  ['tt_5fgetmyuserdata_781',['TT_GetMyUserData',['../group__users.html#gae8dd6c7fbdeead08735e9ce83e16d1b2',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2725:    TEAMTALKDLL_API INT32 TT_GetMyUserData(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:279:        unsafe { ffi::api().TT_GetMyUserData(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMyUserID`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:352:<li>TT_GetMyUserID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:406:<li>TT_GetMyUserID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:394:<li><a class="el" href="struct_t_t_message.html#ad3a853493b3aa2159ded42aaf6358498" title="The source of the event depends on wmMsg.">TTMessage.nSource</a> The client instance's user ID, i.e. what can now be retrieved through <a class="el" href="group__users.html#ga6f56130dc2dc870d6fe279569dd49689" title="Get the local client instance&#39;s user ID.">TT_GetMyUserID()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:207:<tr class="memitem:ga6f56130dc2dc870d6fe279569dd49689"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__users.html#ga6f56130dc2dc870d6fe279569dd49689">TT_GetMyUserID</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:631:<h2 class="memtitle"><span class="permalink"><a href="#ga6f56130dc2dc870d6fe279569dd49689">&#9670;&nbsp;</a></span>TT_GetMyUserID()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:637:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_GetMyUserID </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.js:106:    [ "TT_GetMyUserID", "group__users.html#ga6f56130dc2dc870d6fe279569dd49689", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:118:  ['tt_5fgetmyuserid_782',['TT_GetMyUserID',['../group__users.html#ga6f56130dc2dc870d6fe279569dd49689',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2712:    TEAMTALKDLL_API INT32 TT_GetMyUserID(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:297:        unsafe { ffi::api().TT_GetMyUserID(ptr) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMyUserRights`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1690:<li><a class="el" href="group__users.html#ga8d8a0be51387862fa4e58004b4a388a2" title="Convenience method for TT_GetMyUserAccount()">TT_GetMyUserRights()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:355:<li>TT_GetMyUserRights()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:409:<li>TT_GetMyUserRights()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.js:109:    [ "TT_GetMyUserRights", "group__users.html#ga8d8a0be51387862fa4e58004b4a388a2", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:216:<tr class="memitem:ga8d8a0be51387862fa4e58004b4a388a2"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="group__server.html#ga98e90d0c0ce6fd2bde49e6a95f9df44c">UserRights</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__users.html#ga8d8a0be51387862fa4e58004b4a388a2">TT_GetMyUserRights</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:731:<h2 class="memtitle"><span class="permalink"><a href="#ga8d8a0be51387862fa4e58004b4a388a2">&#9670;&nbsp;</a></span>TT_GetMyUserRights()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:737:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="group__server.html#ga98e90d0c0ce6fd2bde49e6a95f9df44c">UserRights</a> TT_GetMyUserRights </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:119:  ['tt_5fgetmyuserrights_783',['TT_GetMyUserRights',['../group__users.html#ga8d8a0be51387862fa4e58004b4a388a2',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2722:    TEAMTALKDLL_API UserRights TT_GetMyUserRights(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:274:        unsafe { ffi::api().TT_GetMyUserRights(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetMyUserType`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:358:<li>TT_GetMyUserType()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:412:<li>TT_GetMyUserType()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:213:<tr class="memitem:gaa2b3137dbaaa2b0a6bba26e4f79b277a"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="group__users.html#ga8a0faaedbc83d50d383b27f58301a1b4">UserTypes</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__users.html#gaa2b3137dbaaa2b0a6bba26e4f79b277a">TT_GetMyUserType</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:698:<h2 class="memtitle"><span class="permalink"><a href="#gaa2b3137dbaaa2b0a6bba26e4f79b277a">&#9670;&nbsp;</a></span>TT_GetMyUserType()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:704:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="group__users.html#ga8a0faaedbc83d50d383b27f58301a1b4">UserTypes</a> TT_GetMyUserType </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.js:108:    [ "TT_GetMyUserType", "group__users.html#gaa2b3137dbaaa2b0a6bba26e4f79b277a", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:120:  ['tt_5fgetmyusertype_784',['TT_GetMyUserType',['../group__users.html#gaa2b3137dbaaa2b0a6bba26e4f79b277a',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\functions_0.js:93:  ['tt_5fgetmyusertype_1245',['TT_GetMyUserType',['../group__users.html#gaa2b3137dbaaa2b0a6bba26e4f79b277a',1,'TeamTalk.h']]],
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2719:    TEAMTALKDLL_API UserTypes TT_GetMyUserType(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:269:        unsafe { ffi::api().TT_GetMyUserType(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetRootChannelID`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:415:<li>TT_GetRootChannelID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:361:<li>TT_GetRootChannelID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:72:    [ "TT_GetRootChannelID", "group__channels.html#gafef3f33d1fc8629ec06a102a41261c7c", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:179:<tr class="memitem:gafef3f33d1fc8629ec06a102a41261c7c"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#gafef3f33d1fc8629ec06a102a41261c7c">TT_GetRootChannelID</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:577:<h2 class="memtitle"><span class="permalink"><a href="#gafef3f33d1fc8629ec06a102a41261c7c">&#9670;&nbsp;</a></span>TT_GetRootChannelID()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:583:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_GetRootChannelID </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:121:  ['tt_5fgetrootchannelid_785',['TT_GetRootChannelID',['../group__channels.html#gafef3f33d1fc8629ec06a102a41261c7c',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\functions_0.js:94:  ['tt_5fgetrootchannelid_1246',['TT_GetRootChannelID',['../group__channels.html#gafef3f33d1fc8629ec06a102a41261c7c',1,'TeamTalk.h']]],
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2661:    TEAMTALKDLL_API INT32 TT_GetRootChannelID(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:162:        let root = ChannelId(unsafe { ffi::api().TT_GetRootChannelID(self.ptr.0) });
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:85:        ChannelId(unsafe { ffi::api().TT_GetRootChannelID(self.ptr.0) })
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetServerChannels`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1758:<li><a class="el" href="group__channels.html#gae11889f29537de22d516b975590e3212" title="Get all the channels on the server.">TT_GetServerChannels()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2244:<li><a class="el" href="group__channels.html#gae11889f29537de22d516b975590e3212" title="Get all the channels on the server.">TT_GetServerChannels()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:364:<li>TT_GetServerChannels()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:418:<li>TT_GetServerChannels()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:81:    [ "TT_GetServerChannels", "group__channels.html#gae11889f29537de22d516b975590e3212", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:206:<tr class="memitem:gae11889f29537de22d516b975590e3212"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#gae11889f29537de22d516b975590e3212">TT_GetServerChannels</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN OUT <a class="el" href="struct_channel.html">Channel</a> *lpChannels, IN OUT INT32 *lpnHowMany)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:963:<h2 class="memtitle"><span class="permalink"><a href="#gae11889f29537de22d516b975590e3212">&#9670;&nbsp;</a></span>TT_GetServerChannels()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:969:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetServerChannels </td>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2704:    TEAMTALKDLL_API TTBOOL TT_GetServerChannels(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:31:            ffi::api().TT_GetServerChannels(self.ptr.0, std::ptr::null_mut(), &mut count);
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:33:            if ffi::api().TT_GetServerChannels(self.ptr.0, channels.as_mut_ptr(), &mut count) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetServerProperties`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2562:<li>Call <a class="el" href="group__server.html#ga62cfa9875421af897eb080ddff820c0e" title="Get the server&#39;s properties.">TT_GetServerProperties</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2566:<li>Call <a class="el" href="group__server.html#ga62cfa9875421af897eb080ddff820c0e" title="Get the server&#39;s properties.">TT_GetServerProperties</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:367:<li>TT_GetServerProperties()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:421:<li>TT_GetServerProperties()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__commands.html:1427:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__server.html#ga62cfa9875421af897eb080ddff820c0e" title="Get the server&#39;s properties.">TT_GetServerProperties</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:238:<tr class="memitem:ga62cfa9875421af897eb080ddff820c0e"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__server.html#ga62cfa9875421af897eb080ddff820c0e">TT_GetServerProperties</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, OUT <a class="el" href="struct_server_properties.html">ServerProperties</a> *lpServerProperties)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:276:<p><a class="el" href="struct_server_properties.html" title="A struct containing the properties of the server&#39;s settings.">ServerProperties</a> holds the user rights in its <em>uUserRights</em> member variable and is retrieved by calling <a class="el" href="group__server.html#ga62cfa9875421af897eb080ddff820c0e" title="Get the server&#39;s properties.">TT_GetServerProperties</a> once connected to the server.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:279:<a class="el" href="group__server.html#ga62cfa9875421af897eb080ddff820c0e" title="Get the server&#39;s properties.">TT_GetServerProperties</a> </dd></dl>
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2649:    TEAMTALKDLL_API TTBOOL TT_GetServerProperties(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:20:        if unsafe { ffi::api().TT_GetServerProperties(self.ptr.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetServerUsers`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1746:<li><a class="el" href="group__server.html#ga23b11fb239fd7b4c9fcdeb9945bbf35b" title="Get all the users on the server.">TT_GetServerUsers()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2388:<p>All users who are connected to a server can now be retrieved using <a class="el" href="group__server.html#ga23b11fb239fd7b4c9fcdeb9945bbf35b" title="Get all the users on the server.">TT_GetServerUsers()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:424:<li>TT_GetServerUsers()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:995:<p>Use <a class="el" href="group__channels.html#gabdca08af83e08dd77bcd62077f30b638" title="Get the channel with a specific ID.">TT_GetChannel()</a> to get more information about each of the channels. </p><dl class="section see"><dt>See also</dt><dd><a class="el" href="group__server.html#ga23b11fb239fd7b4c9fcdeb9945bbf35b" title="Get all the users on the server.">TT_GetServerUsers()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:370:<li>TT_GetServerUsers()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.js:143:    [ "TT_GetServerUsers", "group__server.html#ga23b11fb239fd7b4c9fcdeb9945bbf35b", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:241:<tr class="memitem:ga23b11fb239fd7b4c9fcdeb9945bbf35b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__server.html#ga23b11fb239fd7b4c9fcdeb9945bbf35b">TT_GetServerUsers</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN OUT <a class="el" href="struct_user.html">User</a> *lpUsers, IN OUT INT32 *lpnHowMany)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:720:<h2 class="memtitle"><span class="permalink"><a href="#ga23b11fb239fd7b4c9fcdeb9945bbf35b">&#9670;&nbsp;</a></span>TT_GetServerUsers()</h2>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2653:    TEAMTALKDLL_API TTBOOL TT_GetServerUsers(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:31:            ffi::api().TT_GetServerUsers(self.ptr.0, std::ptr::null_mut(), &mut count);
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:33:            if ffi::api().TT_GetServerUsers(self.ptr.0, users.as_mut_ptr(), &mut count) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetSoundDeviceEffects`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:719:<li>New function <a class="el" href="group__sounddevices.html#ga20390388ab78450fac451a933c95e5a5" title="Get the audio effects that are currently enabled.">TT_GetSoundDeviceEffects()</a> for getting the <a class="el" href="struct_sound_device_effects.html" title="Set up audio effects supported by the sound device.">SoundDeviceEffects</a> on a client instance.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:373:<li>TT_GetSoundDeviceEffects()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:427:<li>TT_GetSoundDeviceEffects()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:97:    [ "TT_GetSoundDeviceEffects", "group__sounddevices.html#ga20390388ab78450fac451a933c95e5a5", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:270:<tr class="memitem:ga20390388ab78450fac451a933c95e5a5"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga20390388ab78450fac451a933c95e5a5">TT_GetSoundDeviceEffects</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, OUT <a class="el" href="struct_sound_device_effects.html">SoundDeviceEffects</a> *lpSoundDeviceEffect)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1637:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga20390388ab78450fac451a933c95e5a5" title="Get the audio effects that are currently enabled.">TT_GetSoundDeviceEffects()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1642:<h2 class="memtitle"><span class="permalink"><a href="#ga20390388ab78450fac451a933c95e5a5">&#9670;&nbsp;</a></span>TT_GetSoundDeviceEffects()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1648:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetSoundDeviceEffects </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2070:    TEAMTALKDLL_API TTBOOL TT_GetSoundDeviceEffects(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:251:        if unsafe { ffi::api().TT_GetSoundDeviceEffects(self.ptr.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetSoundDevices`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:654:<p>A new <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a> will appear on Android when calling <a class="el" href="group__sounddevices.html#ga55d04cde2114bcb34228fa46142b727a" title="Retrieve list of sound devices for recording and playback.">TT_GetSoundDevices()</a>. This sound device behaves the same as calling Android Java class android.media.AudioManager.setMode(AudioManager.MODE_IN_COMMUNICATION).</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1189:<li><code>inputSampleRates</code> and <code>outputSampleRates</code> replaced by <code>supportedSampleRates</code> due to change of <a class="el" href="group__sounddevices.html#ga55d04cde2114bcb34228fa46142b727a" title="Retrieve list of sound devices for recording and playback.">TT_GetSoundDevices()</a> instead of <code>TT_GetSoundInputDevices()</code> and <code>TT_GetSoundOutputDevices()</code>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1626:<li><a class="el" href="group__sounddevices.html#ga55d04cde2114bcb34228fa46142b727a" title="Retrieve list of sound devices for recording and playback.">TT_GetSoundDevices()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1783:<li>Use <a class="el" href="group__sounddevices.html#ga55d04cde2114bcb34228fa46142b727a" title="Retrieve list of sound devices for recording and playback.">TT_GetSoundDevices()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1787:<li>Use <a class="el" href="group__sounddevices.html#ga55d04cde2114bcb34228fa46142b727a" title="Retrieve list of sound devices for recording and playback.">TT_GetSoundDevices()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:376:<li>TT_GetSoundDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:430:<li>TT_GetSoundDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:747:<a class="el" href="group__sounddevices.html#ga55d04cde2114bcb34228fa46142b727a" title="Retrieve list of sound devices for recording and playback.">TT_GetSoundDevices()</a> </dd></dl>
... (30 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2007:    TEAMTALKDLL_API TTBOOL TT_GetSoundDevices(IN OUT SoundDevice* lpSoundDevices,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:42:            ffi::api().TT_GetSoundDevices(std::ptr::null_mut(), &mut count);
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:44:            if ffi::api().TT_GetSoundDevices(devices.as_mut_ptr(), &mut count) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetSoundInputGainLevel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2518:<li>Call <a class="el" href="group__sounddevices.html#gaf9d3a0b7a8c0d8665453b3647ab73c22" title="Get voice gain level of outgoing audio.">TT_GetSoundInputGainLevel</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:100:<dt>Member <a class="el" href="group__sounddevices.html#gaf9d3a0b7a8c0d8665453b3647ab73c22">TT_GetSoundInputGainLevel</a>  (IN TTInstance *lpTTInstance)</dt>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:433:<li>TT_GetSoundInputGainLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:379:<li>TT_GetSoundInputGainLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:279:<tr class="memitem:gaf9d3a0b7a8c0d8665453b3647ab73c22"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gaf9d3a0b7a8c0d8665453b3647ab73c22">TT_GetSoundInputGainLevel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:888:<a class="el" href="group__sounddevices.html#gaf9d3a0b7a8c0d8665453b3647ab73c22" title="Get voice gain level of outgoing audio.">TT_GetSoundInputGainLevel</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:894:<a class="el" href="group__sounddevices.html#gaf9d3a0b7a8c0d8665453b3647ab73c22" title="Get voice gain level of outgoing audio.">TT_GetSoundInputGainLevel</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:900:<a class="el" href="group__sounddevices.html#gaf9d3a0b7a8c0d8665453b3647ab73c22" title="Get voice gain level of outgoing audio.">TT_GetSoundInputGainLevel</a> </dd></dl>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2081:    TEAMTALKDLL_API INT32 TT_GetSoundInputGainLevel(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:145:        unsafe { ffi::api().TT_GetSoundInputGainLevel(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetSoundInputLevel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2490:<li>Call <a class="el" href="group__sounddevices.html#ga90d91ec066da5de7a6809fa2c43da3bd" title="Get the volume level of the current recorded audio.">TT_GetSoundInputLevel</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:382:<li>TT_GetSoundInputLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:436:<li>TT_GetSoundInputLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:657:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga90d91ec066da5de7a6809fa2c43da3bd" title="Get the volume level of the current recorded audio.">TT_GetSoundInputLevel()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:98:    [ "TT_GetSoundInputLevel", "group__sounddevices.html#ga90d91ec066da5de7a6809fa2c43da3bd", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:273:<tr class="memitem:ga90d91ec066da5de7a6809fa2c43da3bd"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga90d91ec066da5de7a6809fa2c43da3bd">TT_GetSoundInputLevel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:848:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga90d91ec066da5de7a6809fa2c43da3bd" title="Get the volume level of the current recorded audio.">TT_GetSoundInputLevel</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:855:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga90d91ec066da5de7a6809fa2c43da3bd" title="Get the volume level of the current recorded audio.">TT_GetSoundInputLevel</a> </dd>
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2074:    TEAMTALKDLL_API INT32 TT_GetSoundInputLevel(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:135:        unsafe { ffi::api().TT_GetSoundInputLevel(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetSoundInputPreprocess`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:101:<dd><a class="anchor" id="_deprecated000006"></a>Use <a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocessEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:102:<dt>Member <a class="el" href="group__sounddevices.html#gaa305abce71caac22d47acd07fdb6becd">TT_GetSoundInputPreprocess</a>  (IN TTInstance *lpTTInstance, OUT <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a> *lpSpeexDSP)</dt>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:103:<dd><a class="anchor" id="_deprecated000008"></a>Use <a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocessEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:721:<li>New function <a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocessEx()</a> for getting the active <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1634:<li><a class="el" href="group__sounddevices.html#gaa305abce71caac22d47acd07fdb6becd" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocess()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1823:<li>Use <a class="el" href="group__sounddevices.html#gaa305abce71caac22d47acd07fdb6becd" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocess()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:385:<li>TT_GetSoundInputPreprocess()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:388:<li>TT_GetSoundInputPreprocessEx()
... (24 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2088:    TEAMTALKDLL_API TTBOOL TT_GetSoundInputPreprocess(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2096:    TEAMTALKDLL_API TTBOOL TT_GetSoundInputPreprocessEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:236:        if unsafe { ffi::api().TT_GetSoundInputPreprocessEx(self.ptr.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetSoundInputPreprocessEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:101:<dd><a class="anchor" id="_deprecated000006"></a>Use <a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocessEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:103:<dd><a class="anchor" id="_deprecated000008"></a>Use <a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocessEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:721:<li>New function <a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocessEx()</a> for getting the active <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:388:<li>TT_GetSoundInputPreprocessEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:442:<li>TT_GetSoundInputPreprocessEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:104:    [ "TT_GetSoundInputPreprocessEx", "group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:291:<tr class="memitem:gabed6dd7cdc1837d14b8a568211d28da6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6">TT_GetSoundInputPreprocessEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, OUT <a class="el" href="struct_audio_preprocessor.html">AudioPreprocessor</a> *lpAudioPreprocessor)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1759:<dl class="deprecated"><dt><b><a class="el" href="deprecated.html#_deprecated000006">Deprecated:</a></b></dt><dd>Use <a class="el" href="group__sounddevices.html#gabed6dd7cdc1837d14b8a568211d28da6" title="Get the sound preprocessor settings which are currently in use for recorded sound input device (voice...">TT_GetSoundInputPreprocessEx()</a></dd></dl>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2096:    TEAMTALKDLL_API TTBOOL TT_GetSoundInputPreprocessEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:236:        if unsafe { ffi::api().TT_GetSoundInputPreprocessEx(self.ptr.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetSoundOutputVolume`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2498:<li>Call <a class="el" href="group__sounddevices.html#gaff586cd2312dd6b4e0f6292a6c9179eb" title="Get master volume.">TT_GetSoundOutputVolume</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:391:<li>TT_GetSoundOutputVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:445:<li>TT_GetSoundOutputVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:106:    [ "TT_GetSoundOutputVolume", "group__sounddevices.html#gaff586cd2312dd6b4e0f6292a6c9179eb", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:297:<tr class="memitem:gaff586cd2312dd6b4e0f6292a6c9179eb"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gaff586cd2312dd6b4e0f6292a6c9179eb">TT_GetSoundOutputVolume</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:864:<a class="el" href="group__sounddevices.html#gaff586cd2312dd6b4e0f6292a6c9179eb" title="Get master volume.">TT_GetSoundOutputVolume</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:873:<a class="el" href="group__sounddevices.html#gaff586cd2312dd6b4e0f6292a6c9179eb" title="Get master volume.">TT_GetSoundOutputVolume</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:880:<a class="el" href="group__sounddevices.html#gaff586cd2312dd6b4e0f6292a6c9179eb" title="Get master volume.">TT_GetSoundOutputVolume</a> </dd>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2104:    TEAMTALKDLL_API INT32 TT_GetSoundOutputVolume(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:155:        unsafe { ffi::api().TT_GetSoundOutputVolume(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetUser`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:640:<li>New function <a class="el" href="group__sounddevices.html#ga64d749744382bea77b3aeed550f3179d" title="Get the de-jitter configuration for a user.">TT_GetUserJitterControl()</a> for retrieving <a class="el" href="struct_jitter_config.html" title="Configuration parameters for the Jitter Buffer.">JitterConfig</a> on a <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1651:<li>Replacement for <code>TT_GetUserVideoFrame</code> and <code>TT_AcquireUserVideoFrame</code>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1667:<li>Replacement for TT_GetUserDesktopWindow().</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1671:<li>Replacement for TT_GetUserDesktopWindow().</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1675:<li>Replacement for TT_GetUserDesktopWindow().</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1826:<li><code>TT_GetUserVideoFrame</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1866:<li><code>TT_GetUserDesktopCursor</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1870:<li><code>TT_GetUserDesktopInput</code> <ul>
... (74 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2728:    TEAMTALKDLL_API TTBOOL TT_GetUser(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2732:    TEAMTALKDLL_API TTBOOL TT_GetUserStatistics(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2736:    TEAMTALKDLL_API TTBOOL TT_GetUserByUsername(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2768:     TEAMTALKDLL_API TTBOOL TT_GetUserJitterControl(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:301:        unsafe { ffi::api().TT_GetUser(ptr, user_id, user) == 1 }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:408:            ffi::api().TT_GetUserJitterControl(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:432:        if unsafe { ffi::api().TT_GetUser(self.ptr.0, user_id.0, &mut raw) } == 1 {
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:442:        if unsafe { ffi::api().TT_GetUserByUsername(self.ptr.0, username.tt().as_ptr(), &mut raw) }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:454:        if unsafe { ffi::api().TT_GetUserStatistics(self.ptr.0, user_id.0, &mut raw) } == 1 {
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:623:        if unsafe { ffi::api().TT_GetUser(self.ptr.0, my_id.0, &mut user) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetUserByUsername`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:451:<li>TT_GetUserByUsername()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:397:<li>TT_GetUserByUsername()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:228:<tr class="memitem:gaa0394704d4f0f564c86133e7cc586a9b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__users.html#gaa0394704d4f0f564c86133e7cc586a9b">TT_GetUserByUsername</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szUsername, OUT <a class="el" href="struct_user.html">User</a> *lpUser)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:826:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__users.html#gaa0394704d4f0f564c86133e7cc586a9b" title="Get the user with the specified username.">TT_GetUserByUsername</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:875:<h2 class="memtitle"><span class="permalink"><a href="#gaa0394704d4f0f564c86133e7cc586a9b">&#9670;&nbsp;</a></span>TT_GetUserByUsername()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:881:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetUserByUsername </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.js:113:    [ "TT_GetUserByUsername", "group__users.html#gaa0394704d4f0f564c86133e7cc586a9b", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:133:  ['tt_5fgetuserbyusername_797',['TT_GetUserByUsername',['../group__users.html#gaa0394704d4f0f564c86133e7cc586a9b',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2736:    TEAMTALKDLL_API TTBOOL TT_GetUserByUsername(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:442:        if unsafe { ffi::api().TT_GetUserByUsername(self.ptr.0, username.tt().as_ptr(), &mut raw) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetUserJitterControl`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:640:<li>New function <a class="el" href="group__sounddevices.html#ga64d749744382bea77b3aeed550f3179d" title="Get the de-jitter configuration for a user.">TT_GetUserJitterControl()</a> for retrieving <a class="el" href="struct_jitter_config.html" title="Configuration parameters for the Jitter Buffer.">JitterConfig</a> on a <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:400:<li>TT_GetUserJitterControl()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:454:<li>TT_GetUserJitterControl()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:116:    [ "TT_GetUserJitterControl", "group__sounddevices.html#ga64d749744382bea77b3aeed550f3179d", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:327:<tr class="memitem:ga64d749744382bea77b3aeed550f3179d"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga64d749744382bea77b3aeed550f3179d">TT_GetUserJitterControl</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga8a65141d9ea4bf9d2e2377ed6b888a1d">StreamType</a> nStreamType, IN <a class="el" href="struct_jitter_config.html">JitterConfig</a> *lpJitterConfig)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2483:<h2 class="memtitle"><span class="permalink"><a href="#ga64d749744382bea77b3aeed550f3179d">&#9670;&nbsp;</a></span>TT_GetUserJitterControl()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2489:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetUserJitterControl </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:134:  ['tt_5fgetuserjittercontrol_798',['TT_GetUserJitterControl',['../group__sounddevices.html#ga64d749744382bea77b3aeed550f3179d',1,'TeamTalk.h']]],
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2768:     TEAMTALKDLL_API TTBOOL TT_GetUserJitterControl(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:408:            ffi::api().TT_GetUserJitterControl(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetUserStatistics`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:403:<li>TT_GetUserStatistics()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:457:<li>TT_GetUserStatistics()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.js:112:    [ "TT_GetUserStatistics", "group__users.html#ga64465f17e5c4f5bb98b495297ee343fc", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:225:<tr class="memitem:ga64465f17e5c4f5bb98b495297ee343fc"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__users.html#ga64465f17e5c4f5bb98b495297ee343fc">TT_GetUserStatistics</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, OUT <a class="el" href="struct_user_statistics.html">UserStatistics</a> *lpUserStatistics)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:406:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__users.html#ga64465f17e5c4f5bb98b495297ee343fc" title="Get statistics for data and packet reception from a user.">TT_GetUserStatistics</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:831:<h2 class="memtitle"><span class="permalink"><a href="#ga64465f17e5c4f5bb98b495297ee343fc">&#9670;&nbsp;</a></span>TT_GetUserStatistics()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:837:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_GetUserStatistics </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:135:  ['tt_5fgetuserstatistics_799',['TT_GetUserStatistics',['../group__users.html#ga64465f17e5c4f5bb98b495297ee343fc',1,'TeamTalk.h']]],
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2732:    TEAMTALKDLL_API TTBOOL TT_GetUserStatistics(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\users.rs:454:        if unsafe { ffi::api().TT_GetUserStatistics(self.ptr.0, user_id.0, &mut raw) } == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetVersion`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:406:<li>TT_GetVersion()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:460:<li>TT_GetVersion()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:144:<tr class="memitem:ga929c494d8dbcfd28712744bdde51270c"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__initclient.html#ga929c494d8dbcfd28712744bdde51270c">TT_GetVersion</a> (void)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:312:<h2 class="memtitle"><span class="permalink"><a href="#ga929c494d8dbcfd28712744bdde51270c">&#9670;&nbsp;</a></span>TT_GetVersion()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:318:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a>* TT_GetVersion </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.js:28:    [ "TT_GetVersion", "group__initclient.html#ga929c494d8dbcfd28712744bdde51270c", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:136:  ['tt_5fgetversion_800',['TT_GetVersion',['../group__initclient.html#ga929c494d8dbcfd28712744bdde51270c',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:227:<tr class="memdesc:a9002ea7d962f09881c532abecb6fb227"><td class="mdescLeft">&#160;</td><td class="mdescRight">Ensure the header and DLL are exactly the same version. To get the version of the loaded DLL call <a class="el" href="group__initclient.html#ga929c494d8dbcfd28712744bdde51270c" title="Get the DLL&#39;s version number.">TT_GetVersion()</a>. A remote client's version can be seen in the <em>szVersion</em> member of the <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a>-struct.  <a href="_team_talk_8h.html#a9002ea7d962f09881c532abecb6fb227">More...</a><br /></td></tr>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1961:    TEAMTALKDLL_API const TTCHAR* TT_GetVersion(void);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:796:            let ptr = ffi::api().TT_GetVersion();
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:10:            let ptr = ffi::api().TT_GetVersion();
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetVideoCaptureDevices`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1718:<li><a class="el" href="group__videocapture.html#gaf31bb6d2b6fa7d50c3f35c484962ca54" title="Get the list of devices available for video capture.">TT_GetVideoCaptureDevices()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2420:<li>Call <a class="el" href="group__videocapture.html#gaf31bb6d2b6fa7d50c3f35c484962ca54" title="Get the list of devices available for video capture.">TT_GetVideoCaptureDevices</a> to get a list of available capture devices.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:463:<li>TT_GetVideoCaptureDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:409:<li>TT_GetVideoCaptureDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.js:36:    [ "TT_GetVideoCaptureDevices", "group__videocapture.html#gaf31bb6d2b6fa7d50c3f35c484962ca54", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:143:<tr class="memitem:gaf31bb6d2b6fa7d50c3f35c484962ca54"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__videocapture.html#gaf31bb6d2b6fa7d50c3f35c484962ca54">TT_GetVideoCaptureDevices</a> (IN OUT <a class="el" href="struct_video_capture_device.html">VideoCaptureDevice</a> *lpVideoDevices, IN OUT INT32 *lpnHowMany)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:175:<p>To capture video the user application must first query the available capture devices by calling <a class="el" href="group__videocapture.html#gaf31bb6d2b6fa7d50c3f35c484962ca54" title="Get the list of devices available for video capture.">TT_GetVideoCaptureDevices</a>. A <a class="el" href="struct_video_capture_device.html" title="A struct containing the properties of a video capture device.">VideoCaptureDevice</a> supports a certain number of capture formats each described in the <em>videoFormats</em> member of <a class="el" href="struct_video_format.html" title="A struct containing the properties of a video capture format.">VideoFormat</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:266:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__videocapture.html#gaf31bb6d2b6fa7d50c3f35c484962ca54" title="Get the list of devices available for video capture.">TT_GetVideoCaptureDevices</a> </dd></dl>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2198:    TEAMTALKDLL_API TTBOOL TT_GetVideoCaptureDevices(IN OUT VideoCaptureDevice* lpVideoDevices,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:34:            ffi::api().TT_GetVideoCaptureDevices(std::ptr::null_mut(), &mut count);
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:36:            if ffi::api().TT_GetVideoCaptureDevices(devices.as_mut_ptr(), &mut count) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetVoiceActivationLevel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:412:<li>TT_GetVoiceActivationLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:466:<li>TT_GetVoiceActivationLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:22:    [ "TT_GetVoiceActivationLevel", "group__transmission.html#gae1ebca98a70bc217d679c9de5839d6f0", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:141:<tr class="memitem:gae1ebca98a70bc217d679c9de5839d6f0"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#gae1ebca98a70bc217d679c9de5839d6f0">TT_GetVoiceActivationLevel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:442:<a class="el" href="group__transmission.html#gae1ebca98a70bc217d679c9de5839d6f0" title="Get voice activation level.">TT_GetVoiceActivationLevel</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:449:<h2 class="memtitle"><span class="permalink"><a href="#gae1ebca98a70bc217d679c9de5839d6f0">&#9670;&nbsp;</a></span>TT_GetVoiceActivationLevel()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:455:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_GetVoiceActivationLevel </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:852:<a class="el" href="group__transmission.html#gae1ebca98a70bc217d679c9de5839d6f0" title="Get voice activation level.">TT_GetVoiceActivationLevel</a> </dd></dl>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2151:    TEAMTALKDLL_API INT32 TT_GetVoiceActivationLevel(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:215:        unsafe { ffi::api().TT_GetVoiceActivationLevel(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_GetVoiceActivationStopDelay`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2241:<li><a class="el" href="group__transmission.html#gad3d1bf08fc7642e2bc7c261ffccbf506" title="Get the delay of when voice active state should be disabled.">TT_GetVoiceActivationStopDelay()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:415:<li>TT_GetVoiceActivationStopDelay()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:469:<li>TT_GetVoiceActivationStopDelay()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:147:<tr class="memitem:gad3d1bf08fc7642e2bc7c261ffccbf506"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#gad3d1bf08fc7642e2bc7c261ffccbf506">TT_GetVoiceActivationStopDelay</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:513:<h2 class="memtitle"><span class="permalink"><a href="#gad3d1bf08fc7642e2bc7c261ffccbf506">&#9670;&nbsp;</a></span>TT_GetVoiceActivationStopDelay()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:519:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_GetVoiceActivationStopDelay </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:24:    [ "TT_GetVoiceActivationStopDelay", "group__transmission.html#gad3d1bf08fc7642e2bc7c261ffccbf506", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:139:  ['tt_5fgetvoiceactivationstopdelay_803',['TT_GetVoiceActivationStopDelay',['../group__transmission.html#gad3d1bf08fc7642e2bc7c261ffccbf506',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2158:    TEAMTALKDLL_API INT32 TT_GetVoiceActivationStopDelay(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:225:        unsafe { ffi::api().TT_GetVoiceActivationStopDelay(self.ptr.0) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_HotKey_GetKeyString`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:418:<li>TT_HotKey_GetKeyString()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:472:<li>TT_HotKey_GetKeyString()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:677:<p>Use <a class="el" href="group__hotkey.html#ga06fb53fc9f92711a208cd63f17d235bb" title="Get a string description of the virtual-key code.">TT_HotKey_GetKeyString</a> to get a key description of the pressed key.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.js:8:    [ "TT_HotKey_GetKeyString", "group__hotkey.html#ga06fb53fc9f92711a208cd63f17d235bb", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:112:<tr class="memitem:ga06fb53fc9f92711a208cd63f17d235bb"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__hotkey.html#ga06fb53fc9f92711a208cd63f17d235bb">TT_HotKey_GetKeyString</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nVKCode, OUT <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szKeyName[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:333:<h2 class="memtitle"><span class="permalink"><a href="#ga06fb53fc9f92711a208cd63f17d235bb">&#9670;&nbsp;</a></span>TT_HotKey_GetKeyString()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:339:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_HotKey_GetKeyString </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:140:  ['tt_5fhotkey_5fgetkeystring_804',['TT_HotKey_GetKeyString',['../group__hotkey.html#ga06fb53fc9f92711a208cd63f17d235bb',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2889:    TEAMTALKDLL_API TTBOOL TT_HotKey_GetKeyString(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\hotkeys.rs:46:            if ffi::api().TT_HotKey_GetKeyString(self.ptr.0, vk_code, buf.as_mut_ptr()) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_HotKey_InstallTestHook`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:475:<li>TT_HotKey_InstallTestHook()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:421:<li>TT_HotKey_InstallTestHook()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:676:<p>When <a class="el" href="group__hotkey.html#gaa93292c27952dae12ee913a7eb1126a2" title="Install a test hook so the HWND will be messaged whenever a key or mouse button is pressed.">TT_HotKey_InstallTestHook</a> is called a hook is installed in Windows which intercepts all keyboard and mouse presses. Every time a key or mouse is pressed or released this event is posted.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:681:<li><a class="el" href="struct_t_t_message.html#a8003a57bf3c8f798a288474699b78dc7" title="Valid if ttType is __TTBOOL.">TTMessage.bActive</a> Placed in union of <a class="el" href="struct_t_t_message.html" title="A struct containing the properties of an event.">TTMessage</a>. TRUE when key is down and FALSE when released. <dl class="section see"><dt>See also</dt><dd><a class="el" href="group__hotkey.html#gaa93292c27952dae12ee913a7eb1126a2" title="Install a test hook so the HWND will be messaged whenever a key or mouse button is pressed.">TT_HotKey_InstallTestHook</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.js:6:    [ "TT_HotKey_InstallTestHook", "group__hotkey.html#gaa93292c27952dae12ee913a7eb1126a2", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:106:<tr class="memitem:gaa93292c27952dae12ee913a7eb1126a2"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__hotkey.html#gaa93292c27952dae12ee913a7eb1126a2">TT_HotKey_InstallTestHook</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN HWND hWnd, UINT32 uMsg)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:110:<tr class="memdesc:ga21f6e90f171fd7cac793baca6b5db1e6"><td class="mdescLeft">&#160;</td><td class="mdescRight">Remove the test hook again so the <em>hWnd</em> in <a class="el" href="group__hotkey.html#gaa93292c27952dae12ee913a7eb1126a2" title="Install a test hook so the HWND will be messaged whenever a key or mouse button is pressed.">TT_HotKey_InstallTestHook</a> will no longer be notified.  <a href="group__hotkey.html#ga21f6e90f171fd7cac793baca6b5db1e6">More...</a><br /></td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:119:<p>Windows supports intercepting key strokes globally, i.e. without having the user application's window focused. To investigate which keys are currently being pressed the function <a class="el" href="group__hotkey.html#gaa93292c27952dae12ee913a7eb1126a2" title="Install a test hook so the HWND will be messaged whenever a key or mouse button is pressed.">TT_HotKey_InstallTestHook</a> can be used. Once the desired key-combination has been found the function <a class="el" href="group__hotkey.html#ga0e8aebb699ec3010e839f11d5007fc15" title="Register a global hotkey.">TT_HotKey_Register</a> can be used to register the combination as a hotkey and have the <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a16fde85ffece1186d1ca2ebf694b045a" title="A hotkey has been acticated or deactivated.">CLIENTEVENT_HOTKEY</a> event posted whenever the key combination becomes active.</p>
... (14 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2882:    TEAMTALKDLL_API TTBOOL TT_HotKey_InstallTestHook(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\hotkeys.rs:32:        unsafe { ffi::api().TT_HotKey_InstallTestHook(self.ptr.0, hwnd, msg) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_HotKey_IsActive`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2630:<li>Call <a class="el" href="group__hotkey.html#ga055650309c64caf656ec336c8917a63f" title="Check whether hotkey is active.">TT_HotKey_IsActive</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:424:<li>TT_HotKey_IsActive()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:478:<li>TT_HotKey_IsActive()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.js:5:    [ "TT_HotKey_IsActive", "group__hotkey.html#ga055650309c64caf656ec336c8917a63f", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:103:<tr class="memitem:ga055650309c64caf656ec336c8917a63f"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__hotkey.html#ga055650309c64caf656ec336c8917a63f">TT_HotKey_IsActive</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nHotKeyID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:220:<h2 class="memtitle"><span class="permalink"><a href="#ga055650309c64caf656ec336c8917a63f">&#9670;&nbsp;</a></span>TT_HotKey_IsActive()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:226:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_HotKey_IsActive </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:142:  ['tt_5fhotkey_5fisactive_806',['TT_HotKey_IsActive',['../group__hotkey.html#ga055650309c64caf656ec336c8917a63f',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2878:    TEAMTALKDLL_API INT32 TT_HotKey_IsActive(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\hotkeys.rs:21:        unsafe { ffi::api().TT_HotKey_IsActive(self.ptr.0, id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_HotKey_Register`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2622:<li>Call <a class="el" href="group__hotkey.html#ga0e8aebb699ec3010e839f11d5007fc15" title="Register a global hotkey.">TT_HotKey_Register</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:427:<li>TT_HotKey_Register()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:481:<li>TT_HotKey_Register()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:667:<li><a class="el" href="struct_t_t_message.html#ad3a853493b3aa2159ded42aaf6358498" title="The source of the event depends on wmMsg.">TTMessage.nSource</a> The hotkey ID passed to <a class="el" href="group__hotkey.html#ga0e8aebb699ec3010e839f11d5007fc15" title="Register a global hotkey.">TT_HotKey_Register()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:671:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__hotkey.html#ga0e8aebb699ec3010e839f11d5007fc15" title="Register a global hotkey.">TT_HotKey_Register</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.js:3:    [ "TT_HotKey_Register", "group__hotkey.html#ga0e8aebb699ec3010e839f11d5007fc15", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:97:<tr class="memitem:ga0e8aebb699ec3010e839f11d5007fc15"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__hotkey.html#ga0e8aebb699ec3010e839f11d5007fc15">TT_HotKey_Register</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nHotKeyID, IN const INT32 *lpnVKCodes, IN INT32 nVKCodeCount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:119:<p>Windows supports intercepting key strokes globally, i.e. without having the user application's window focused. To investigate which keys are currently being pressed the function <a class="el" href="group__hotkey.html#gaa93292c27952dae12ee913a7eb1126a2" title="Install a test hook so the HWND will be messaged whenever a key or mouse button is pressed.">TT_HotKey_InstallTestHook</a> can be used. Once the desired key-combination has been found the function <a class="el" href="group__hotkey.html#ga0e8aebb699ec3010e839f11d5007fc15" title="Register a global hotkey.">TT_HotKey_Register</a> can be used to register the combination as a hotkey and have the <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a16fde85ffece1186d1ca2ebf694b045a" title="A hotkey has been acticated or deactivated.">CLIENTEVENT_HOTKEY</a> event posted whenever the key combination becomes active.</p>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2868:    TEAMTALKDLL_API TTBOOL TT_HotKey_Register(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\hotkeys.rs:9:            ffi::api().TT_HotKey_Register(self.ptr.0, id, vk_codes.as_ptr(), vk_codes.len() as i32)
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_HotKey_RemoveTestHook`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:430:<li>TT_HotKey_RemoveTestHook()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:484:<li>TT_HotKey_RemoveTestHook()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:109:<tr class="memitem:ga21f6e90f171fd7cac793baca6b5db1e6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__hotkey.html#ga21f6e90f171fd7cac793baca6b5db1e6">TT_HotKey_RemoveTestHook</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:299:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__hotkey.html#ga21f6e90f171fd7cac793baca6b5db1e6" title="Remove the test hook again so the hWnd in TT_HotKey_InstallTestHook will no longer be notified.">TT_HotKey_RemoveTestHook</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:306:<h2 class="memtitle"><span class="permalink"><a href="#ga21f6e90f171fd7cac793baca6b5db1e6">&#9670;&nbsp;</a></span>TT_HotKey_RemoveTestHook()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:312:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_HotKey_RemoveTestHook </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.js:7:    [ "TT_HotKey_RemoveTestHook", "group__hotkey.html#ga21f6e90f171fd7cac793baca6b5db1e6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:144:  ['tt_5fhotkey_5fremovetesthook_808',['TT_HotKey_RemoveTestHook',['../group__hotkey.html#ga21f6e90f171fd7cac793baca6b5db1e6',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2886:    TEAMTALKDLL_API TTBOOL TT_HotKey_RemoveTestHook(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\hotkeys.rs:37:        unsafe { ffi::api().TT_HotKey_RemoveTestHook(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_HotKey_Unregister`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2626:<li>Call <a class="el" href="group__hotkey.html#ga4ca9b300f9cb68ff71b951634f2804bf" title="Unregister a registered hotkey.">TT_HotKey_Unregister</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:487:<li>TT_HotKey_Unregister()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:433:<li>TT_HotKey_Unregister()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:673:<a class="el" href="group__hotkey.html#ga4ca9b300f9cb68ff71b951634f2804bf" title="Unregister a registered hotkey.">TT_HotKey_Unregister</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:100:<tr class="memitem:ga4ca9b300f9cb68ff71b951634f2804bf"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__hotkey.html#ga4ca9b300f9cb68ff71b951634f2804bf">TT_HotKey_Unregister</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nHotKeyID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:175:<a class="el" href="group__hotkey.html#ga4ca9b300f9cb68ff71b951634f2804bf" title="Unregister a registered hotkey.">TT_HotKey_Unregister</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:182:<h2 class="memtitle"><span class="permalink"><a href="#ga4ca9b300f9cb68ff71b951634f2804bf">&#9670;&nbsp;</a></span>TT_HotKey_Unregister()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__hotkey.html:188:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_HotKey_Unregister </td>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2874:    TEAMTALKDLL_API TTBOOL TT_HotKey_Unregister(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\hotkeys.rs:16:        unsafe { ffi::api().TT_HotKey_Unregister(self.ptr.0, id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitLocalPlayback`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:593:<li>New enum value <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dabbaecd785019d0eadc798e99d753b32b" title="Stream type for audio of local playback.">STREAMTYPE_LOCALMEDIAPLAYBACK_AUDIO</a> for <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> playback.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:744:<p>Media files can now be played using the TeamTalk instance configured sound output device. Call <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> to playback a media file.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:817:<li>New functions <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a>, <a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback()</a> and <a class="el" href="group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49">TT_StopLocalPlayback()</a> for local media playback.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:837:<li>New <a class="el" href="struct_t_t_audio_preprocessor.html" title="Use TeamTalk&#39;s internal audio preprocessor for gain audio. Same as used for TT_SetSoundInputGainLevel...">TTAudioPreprocessor</a> struct for <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> or <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a>.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:436:<li>TT_InitLocalPlayback()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:490:<li>TT_InitLocalPlayback()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__codecs.html:691:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__codecs.html:820:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> </dd></dl>
... (24 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2261:    TEAMTALKDLL_API INT32 TT_InitLocalPlayback(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:83:            ffi::api().TT_InitLocalPlayback(self.ptr.0, file_path.tt().as_ptr(), &playback.to_ffi())
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitSoundDuplexDevices`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:681:<li>Enable this feature using <a class="el" href="group__sounddevices.html#ga9970ab20d37e0cc2ba3682ea47312946" title="Enable duplex mode where multiple audio streams are mixed into a single stream using software.">TT_InitSoundDuplexDevices()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2281:<p>When initializing the client instance's sound system in duplex mode it is now possible to enable echo cancellation. Note, however, that echo cancellation performs poorly on Windows whereas it's very effective on Mac OS X and Linux. Check out <a class="el" href="group__sounddevices.html#ga9970ab20d37e0cc2ba3682ea47312946" title="Enable duplex mode where multiple audio streams are mixed into a single stream using software.">TT_InitSoundDuplexDevices()</a> and TT_EnableEchoCancellation() on how to use echo cancellation.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2309:<li><a class="el" href="group__sounddevices.html#ga9970ab20d37e0cc2ba3682ea47312946" title="Enable duplex mode where multiple audio streams are mixed into a single stream using software.">TT_InitSoundDuplexDevices()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2330:<li>For echo cancellation to be enabled the sound system must have been initialized in duplex mode using <a class="el" href="group__sounddevices.html#ga9970ab20d37e0cc2ba3682ea47312946" title="Enable duplex mode where multiple audio streams are mixed into a single stream using software.">TT_InitSoundDuplexDevices()</a>. Note that echo cancellation performs poorly on Windows.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:493:<li>TT_InitSoundDuplexDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:439:<li>TT_InitSoundDuplexDevices()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:256:<tr><td class="fieldname"><a id="gga58d6e380015b4b1c92c0f09fd6bcfc1ca3ee0ecf955e9bbe96ebc74094ab17953"></a>CLIENT_SNDINOUTPUT_DUPLEX&#160;</td><td class="fielddoc"><p>If set the client instance is running in sound duplex mode where multiple audio output streams are mixed into a single stream. This option must be enabled to support echo cancellation (see <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocess()</a>). Call <a class="el" href="group__sounddevices.html#ga9970ab20d37e0cc2ba3682ea47312946" title="Enable duplex mode where multiple audio streams are mixed into a single stream using software.">TT_InitSoundDuplexDevices()</a> to enable duplex mode. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:340:<p>Sound input is initialized using <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice()</a> or <a class="el" href="group__sounddevices.html#ga9970ab20d37e0cc2ba3682ea47312946" title="Enable duplex mode where multiple audio streams are mixed into a single stream using software.">TT_InitSoundDuplexDevices()</a>.</p>
... (25 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2052:    TEAMTALKDLL_API TTBOOL TT_InitSoundDuplexDevices(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:99:        unsafe { ffi::api().TT_InitSoundDuplexDevices(self.ptr.0, in_id, out_id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitSoundInputDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:727:<p>Value of <a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095" title="Sound device ID for iOS AudioUnit subtype Voice-Processing I/O Unit.">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a> has been changed to include <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>. Previously the iOS sound device that does voice preprocessing actually ran in its own shared device. However, with the introduction of <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a> in TeamTalk v5.5 it is simpler for iOS to use the same shared device property as on Android. Therefore ensure that <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice()</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice()</a> is not called with 1 instead of the value of <a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095" title="Sound device ID for iOS AudioUnit subtype Voice-Processing I/O Unit.">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:740:<p>To initialize a shared audio input device call like this: </p><pre class="fragment">  TT_InitSoundInputDevice(ttInst, inputid | TT_SOUNDDEVICE_ID_SHARED_FLAG);
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:741:</pre><p> To initialize a shared audio output device call like this: </p><pre class="fragment">  TT_InitSoundInputDevice(ttInst, outputid | TT_SOUNDDEVICE_ID_SHARED_FLAG);
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:766:<p>The <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dae5064c6cd0444d6e4f46598eaf4fb018" title="Voice stream type which is audio recorded from a sound input device.">STREAMTYPE_VOICE</a> could previously only come from the configured <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a> passed to <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice()</a>. Now it's possible to replace the, typically microphone, audio input with a custom audio stream by passing raw audio in <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> to <a class="el" href="group__transmission.html#gac3efce380265ba02123a5388b803be90" title="Transmit application provided raw audio in AudioBlock-structs as STREAMTYPE_VOICE,...">TT_InsertAudioBlock()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2474:<li>Call <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2478:<li>Check <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1ca82b03535e81a26210be47e4f02d6d026" title="If set the client instance&#39;s sound input device has been initialized, i.e. TT_InitSoundInputDevice ha...">CLIENT_SNDINPUT_READY</a> and <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1caba1347d579eb049fb81615bf4ce9b1e6" title="If set the client instance&#39;s sound output device has been initialized, i.e. TT_InitSoundOutputDevice ...">CLIENT_SNDOUTPUT_READY</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2486:<li>Call <a class="el" href="group__sounddevices.html#gaff10e648d33eea6a1561f086db92847e" title="Shutdown the input sound device.">TT_CloseSoundInputDevice</a> and <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice</a> instead. Similar way for output device.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:432:   Call #TT_CloseSoundInputDevice and TT_InitSoundInputDevice
... (43 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2036:    TEAMTALKDLL_API TTBOOL TT_InitSoundInputDevice(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:79:        unsafe { ffi::api().TT_InitSoundInputDevice(self.ptr.0, device_id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitSoundInputSharedDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:689:<p>Now it's possible to manually specify the sample rate and number of audio channels using <a class="el" href="group__sounddevices.html#gae4b76893a9bf02b63b94580f24662698" title="Setup sample rate, channels and frame size of shared sound input device.">TT_InitSoundInputSharedDevice()</a> and <a class="el" href="group__sounddevices.html#ga73512f3c257ddcf138b6128ca9feddc6" title="Setup sample rate, channels and frame size of shared sound output device.">TT_InitSoundOutputSharedDevice()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:716:<li>New function <a class="el" href="group__sounddevices.html#gae4b76893a9bf02b63b94580f24662698" title="Setup sample rate, channels and frame size of shared sound input device.">TT_InitSoundInputSharedDevice()</a> for specifying sample rate, frame size and mono/stereo of the input <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a> that has been initialized with <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:499:<li>TT_InitSoundInputSharedDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:445:<li>TT_InitSoundInputSharedDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:89:    [ "TT_InitSoundInputSharedDevice", "group__sounddevices.html#gae4b76893a9bf02b63b94580f24662698", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:246:<tr class="memitem:gae4b76893a9bf02b63b94580f24662698"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gae4b76893a9bf02b63b94580f24662698">TT_InitSoundInputSharedDevice</a> (IN INT32 nSampleRate, IN INT32 nChannels, IN INT32 nFrameSize)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1290:<h2 class="memtitle"><span class="permalink"><a href="#gae4b76893a9bf02b63b94580f24662698">&#9670;&nbsp;</a></span>TT_InitSoundInputSharedDevice()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1296:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_InitSoundInputSharedDevice </td>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2040:    TEAMTALKDLL_API TTBOOL TT_InitSoundInputSharedDevice(IN INT32 nSampleRate,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:89:        unsafe { ffi::api().TT_InitSoundInputSharedDevice(rate, chans, frame) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitSoundOutputDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:727:<p>Value of <a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095" title="Sound device ID for iOS AudioUnit subtype Voice-Processing I/O Unit.">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a> has been changed to include <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>. Previously the iOS sound device that does voice preprocessing actually ran in its own shared device. However, with the introduction of <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a> in TeamTalk v5.5 it is simpler for iOS to use the same shared device property as on Android. Therefore ensure that <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice()</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice()</a> is not called with 1 instead of the value of <a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095" title="Sound device ID for iOS AudioUnit subtype Voice-Processing I/O Unit.">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2474:<li>Call <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2478:<li>Check <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1ca82b03535e81a26210be47e4f02d6d026" title="If set the client instance&#39;s sound input device has been initialized, i.e. TT_InitSoundInputDevice ha...">CLIENT_SNDINPUT_READY</a> and <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1caba1347d579eb049fb81615bf4ce9b1e6" title="If set the client instance&#39;s sound output device has been initialized, i.e. TT_InitSoundOutputDevice ...">CLIENT_SNDOUTPUT_READY</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:448:<li>TT_InitSoundOutputDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:502:<li>TT_InitSoundOutputDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__errorhandling.html:440:   Call #TT_CloseSoundOutputDevice and TT_InitSoundOutputDevice
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:254:<tr><td class="fieldname"><a id="gga58d6e380015b4b1c92c0f09fd6bcfc1caba1347d579eb049fb81615bf4ce9b1e6"></a>CLIENT_SNDOUTPUT_READY&#160;</td><td class="fielddoc"><p>If set the client instance's sound output device has been initialized, i.e. <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice</a> has been called successfully. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:618:<p>The sound system properties of the <code>lpTTInstance</code> will be used for playback, i.e. <a class="el" href="group__sounddevices.html#gad83a50e6871a13f927cfee08c3e5cdca" title="Set all users mute.">TT_SetSoundOutputMute()</a>, <a class="el" href="group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6" title="Set master volume.">TT_SetSoundOutputVolume()</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice()</a>.</p>
... (35 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2044:    TEAMTALKDLL_API TTBOOL TT_InitSoundOutputDevice(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:84:        unsafe { ffi::api().TT_InitSoundOutputDevice(self.ptr.0, device_id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitSoundOutputSharedDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:689:<p>Now it's possible to manually specify the sample rate and number of audio channels using <a class="el" href="group__sounddevices.html#gae4b76893a9bf02b63b94580f24662698" title="Setup sample rate, channels and frame size of shared sound input device.">TT_InitSoundInputSharedDevice()</a> and <a class="el" href="group__sounddevices.html#ga73512f3c257ddcf138b6128ca9feddc6" title="Setup sample rate, channels and frame size of shared sound output device.">TT_InitSoundOutputSharedDevice()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:717:<li>New function <a class="el" href="group__sounddevices.html#ga73512f3c257ddcf138b6128ca9feddc6" title="Setup sample rate, channels and frame size of shared sound output device.">TT_InitSoundOutputSharedDevice()</a> for specifying sample rate, frame size and mono/stereo of the output <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a> that has been initialized with <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:451:<li>TT_InitSoundOutputSharedDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:505:<li>TT_InitSoundOutputSharedDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:91:    [ "TT_InitSoundOutputSharedDevice", "group__sounddevices.html#ga73512f3c257ddcf138b6128ca9feddc6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:252:<tr class="memitem:ga73512f3c257ddcf138b6128ca9feddc6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga73512f3c257ddcf138b6128ca9feddc6">TT_InitSoundOutputSharedDevice</a> (IN INT32 nSampleRate, IN INT32 nChannels, IN INT32 nFrameSize)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1335:<a class="el" href="group__sounddevices.html#ga73512f3c257ddcf138b6128ca9feddc6" title="Setup sample rate, channels and frame size of shared sound output device.">TT_InitSoundOutputSharedDevice()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1402:<h2 class="memtitle"><span class="permalink"><a href="#ga73512f3c257ddcf138b6128ca9feddc6">&#9670;&nbsp;</a></span>TT_InitSoundOutputSharedDevice()</h2>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2048:    TEAMTALKDLL_API TTBOOL TT_InitSoundOutputSharedDevice(IN INT32 nSampleRate,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:94:        unsafe { ffi::api().TT_InitSoundOutputSharedDevice(rate, chans, frame) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitTeamTalk`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1706:<li><a class="el" href="group__initclient.html#gaea369735ecf5c6c75f5a30944f389bbe" title="Create a new TeamTalk client instance where events are posted to a HWND.">TT_InitTeamTalk()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2385:<p>When using <a class="el" href="group__initclient.html#gaea369735ecf5c6c75f5a30944f389bbe" title="Create a new TeamTalk client instance where events are posted to a HWND.">TT_InitTeamTalk()</a> a <code>HWND</code> is passed which is used for event handling. If at some point another <code>HWND</code> should be used for event handling this <code>HWND</code> can be swapped using <a class="el" href="group__initclient.html#ga5747b70f13343bfec8764183a2b49f63" title="Replace the HWND passed as parameter to TT_InitTeamTalk with this HWND.">TT_SwapTeamTalkHWND()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\examples.html:278:<p>This is a simple console application which stores all audio sent to the server to a specific directory on disk. It also displays all user text chat sessions, file uploads, etc. This example gives a good idea of how events are processed in TeamTalk when using <a class="el" href="group__initclient.html#gaebc89ca414258f4e8228f8af91343e72" title="Create a new TeamTalk client instance where events are &#39;polled&#39; using TT_GetMessage.">TT_InitTeamTalkPoll()</a> and events are not posted to a window handle.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:454:<li>TT_InitTeamTalk()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:457:<li>TT_InitTeamTalkPoll()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:508:<li>TT_InitTeamTalk()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:511:<li>TT_InitTeamTalkPoll()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:595:    <tr><td class="paramname">lpTTInstance</td><td>Pointer to client instance created by <a class="el" href="group__initclient.html#gaea369735ecf5c6c75f5a30944f389bbe" title="Create a new TeamTalk client instance where events are posted to a HWND.">TT_InitTeamTalk</a>. </td></tr>
... (200 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1965:    TEAMTALKDLL_API TTInstance* TT_InitTeamTalk(IN HWND hWnd, IN UINT32 uMsg);
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1973:    TEAMTALKDLL_API TTInstance* TT_InitTeamTalkPoll(void);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\lib.rs:59:/// `TT_InitTeamTalk`/`TT_InitTeamTalkPoll`, which in this crate happens inside
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:18:    /// TeamTalk C-API requires license configuration before `TT_InitTeamTalk`.
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:173:        unsafe { ffi::api().TT_InitTeamTalkPoll() }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:178:        unsafe { ffi::api().TT_InitTeamTalk(hwnd, msg) }
```

### crates/teamtalk/tests
No matches

### docs
```text
D:\downloads\repos\TeamTalkRust\docs\configuration.md:47:`TT_InitTeamTalk`) and ensures the first client instance uses the license.
```

### README.md
No matches

## `TT_InitTeamTalkPoll`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\examples.html:278:<p>This is a simple console application which stores all audio sent to the server to a specific directory on disk. It also displays all user text chat sessions, file uploads, etc. This example gives a good idea of how events are processed in TeamTalk when using <a class="el" href="group__initclient.html#gaebc89ca414258f4e8228f8af91343e72" title="Create a new TeamTalk client instance where events are &#39;polled&#39; using TT_GetMessage.">TT_InitTeamTalkPoll()</a> and events are not posted to a window handle.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:511:<li>TT_InitTeamTalkPoll()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:457:<li>TT_InitTeamTalkPoll()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.js:31:    [ "TT_InitTeamTalkPoll", "group__initclient.html#gaebc89ca414258f4e8228f8af91343e72", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\license.html:121:<div class="line">  <a class="code" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a>* ttclient = <a class="code" href="group__initclient.html#gaebc89ca414258f4e8228f8af91343e72">TT_InitTeamTalkPoll</a>();</div>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\license.html:125:<div class="ttc" id="agroup__initclient_html_gaebc89ca414258f4e8228f8af91343e72"><div class="ttname"><a href="group__initclient.html#gaebc89ca414258f4e8228f8af91343e72">TT_InitTeamTalkPoll</a></div><div class="ttdeci">TEAMTALKDLL_API TTInstance * TT_InitTeamTalkPoll(void)</div><div class="ttdoc">Create a new TeamTalk client instance where events are 'polled' using TT_GetMessage.</div></div>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:153:<tr class="memitem:gaebc89ca414258f4e8228f8af91343e72"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__initclient.html#gaebc89ca414258f4e8228f8af91343e72">TT_InitTeamTalkPoll</a> (void)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:174:<p><a class="el" href="group__initclient.html#gaebc89ca414258f4e8228f8af91343e72" title="Create a new TeamTalk client instance where events are &#39;polled&#39; using TT_GetMessage.">TT_InitTeamTalkPoll()</a> will instantiate a new client instance where events are polled using <a class="el" href="group__initclient.html#ga34fe8de6133a101aa70574225d7dcae0" title="Poll for events in the client instance.">TT_GetMessage()</a>. The events are defined in <a class="el" href="group__events.html#gae7ac512b56742737122b735f65babbd1" title="TeamTalk client event messages.">ClientEvent</a>. On Windows <a class="el" href="group__initclient.html#gaea369735ecf5c6c75f5a30944f389bbe" title="Create a new TeamTalk client instance where events are posted to a HWND.">TT_InitTeamTalk()</a> can also be used which takes as parameter a HWND that will have a message posted whenever an event in the client instance takes place.</p>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1973:    TEAMTALKDLL_API TTInstance* TT_InitTeamTalkPoll(void);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\lib.rs:59:/// `TT_InitTeamTalk`/`TT_InitTeamTalkPoll`, which in this crate happens inside
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:173:        unsafe { ffi::api().TT_InitTeamTalkPoll() }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InitVideoCaptureDevice`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1722:<li><a class="el" href="group__videocapture.html#ga1abc2baee2ae76f8a90a7aee0b9d483a" title="Initialize a video capture device.">TT_InitVideoCaptureDevice()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2421:<li>Call <a class="el" href="group__videocapture.html#ga1abc2baee2ae76f8a90a7aee0b9d483a" title="Initialize a video capture device.">TT_InitVideoCaptureDevice</a> to initialize the video capture device.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:514:<li>TT_InitVideoCaptureDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:460:<li>TT_InitVideoCaptureDevice()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__codecs.html:726:<a class="el" href="group__videocapture.html#ga1abc2baee2ae76f8a90a7aee0b9d483a" title="Initialize a video capture device.">TT_InitVideoCaptureDevice</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:276:<tr><td class="fieldname"><a id="gga58d6e380015b4b1c92c0f09fd6bcfc1caffd34a624af8ed4aa334676a696b1fbf"></a>CLIENT_VIDEOCAPTURE_READY&#160;</td><td class="fielddoc"><p>If set the client instance's video device has been initialized, i.e. <a class="el" href="group__videocapture.html#ga1abc2baee2ae76f8a90a7aee0b9d483a" title="Initialize a video capture device.">TT_InitVideoCaptureDevice</a> has been called successfuly. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:175:<p>To transmit audio the client must have the flag <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1ca82b03535e81a26210be47e4f02d6d026" title="If set the client instance&#39;s sound input device has been initialized, i.e. TT_InitSoundInputDevice ha...">CLIENT_SNDINPUT_READY</a> enabled which is done in the function <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice</a>. To transmit video requires the flag <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1caffd34a624af8ed4aa334676a696b1fbf" title="If set the client instance&#39;s video device has been initialized, i.e. TT_InitVideoCaptureDevice has be...">CLIENT_VIDEOCAPTURE_READY</a> which is enabled by the function <a class="el" href="group__videocapture.html#ga1abc2baee2ae76f8a90a7aee0b9d483a" title="Initialize a video capture device.">TT_InitVideoCaptureDevice</a>. To hear what others users are saying a sound output device must have been configured using <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice</a> and thereby have enabled the flag <a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1caba1347d579eb049fb81615bf4ce9b1e6" title="If set the client instance&#39;s sound output device has been initialized, i.e. TT_InitSoundOutputDevice ...">CLIENT_SNDOUTPUT_READY</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:234:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__videocapture.html#ga1abc2baee2ae76f8a90a7aee0b9d483a" title="Initialize a video capture device.">TT_InitVideoCaptureDevice()</a> </dd></dl>
... (20 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2202:    TEAMTALKDLL_API TTBOOL TT_InitVideoCaptureDevice(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:48:        unsafe { ffi::api().TT_InitVideoCaptureDevice(self.ptr.0, id.as_ptr(), &raw_fmt) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_InsertAudioBlock`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:766:<p>The <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dae5064c6cd0444d6e4f46598eaf4fb018" title="Voice stream type which is audio recorded from a sound input device.">STREAMTYPE_VOICE</a> could previously only come from the configured <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a> passed to <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice()</a>. Now it's possible to replace the, typically microphone, audio input with a custom audio stream by passing raw audio in <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> to <a class="el" href="group__transmission.html#gac3efce380265ba02123a5388b803be90" title="Transmit application provided raw audio in AudioBlock-structs as STREAMTYPE_VOICE,...">TT_InsertAudioBlock()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:767:<p>The audio input must be passed continuesly to <a class="el" href="group__transmission.html#gac3efce380265ba02123a5388b803be90" title="Transmit application provided raw audio in AudioBlock-structs as STREAMTYPE_VOICE,...">TT_InsertAudioBlock()</a> to keep the input going. Use event <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224ac784fb5cfba75f58a081c55456dc3a51" title="Progress is audio being injected as STREAMTYPE_VOICE.">CLIENTEVENT_AUDIOINPUT</a> to monitor progress. The <a class="el" href="struct_audio_input_progress.html" title="The progress of the audio currently being processed as audio input.">AudioInputProgress</a> struct hold information about the queue sizes/progress.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:861:<li>New function <a class="el" href="group__transmission.html#gac3efce380265ba02123a5388b803be90" title="Transmit application provided raw audio in AudioBlock-structs as STREAMTYPE_VOICE,...">TT_InsertAudioBlock()</a> for injecting audio which replaces microphone input.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:463:<li>TT_InsertAudioBlock()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:517:<li>TT_InsertAudioBlock()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:719:<p><code>nStreamID</code> of <a class="el" href="struct_audio_input_progress.html" title="The progress of the audio currently being processed as audio input.">AudioInputProgress</a> is the stream ID provided in the <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> when calling <a class="el" href="group__transmission.html#gac3efce380265ba02123a5388b803be90" title="Transmit application provided raw audio in AudioBlock-structs as STREAMTYPE_VOICE,...">TT_InsertAudioBlock()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:720:<p>When <code>uElapsedMSec</code> and <code>uQueueMSec</code> of <a class="el" href="struct_audio_input_progress.html" title="The progress of the audio currently being processed as audio input.">AudioInputProgress</a> are zero then the stream ID (session) has ended. An audio input session has ended when an empty <a class="el" href="struct_audio_block.html" title="An audio block containing the raw audio from a user who was talking.">AudioBlock</a> has been inserted using <a class="el" href="group__transmission.html#gac3efce380265ba02123a5388b803be90" title="Transmit application provided raw audio in AudioBlock-structs as STREAMTYPE_VOICE,...">TT_InsertAudioBlock()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:335:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__transmission.html#gac3efce380265ba02123a5388b803be90" title="Transmit application provided raw audio in AudioBlock-structs as STREAMTYPE_VOICE,...">TT_InsertAudioBlock()</a> </dd></dl>
... (14 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2135:    TEAMTALKDLL_API TTBOOL TT_InsertAudioBlock(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:435:        unsafe { ffi::api().TT_InsertAudioBlock(self.ptr.0, block) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_IsChannelOperator`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.js:80:    [ "TT_IsChannelOperator", "group__channels.html#gafc207651653dbde2b3cfd6615695df97", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:520:<li>TT_IsChannelOperator()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:466:<li>TT_IsChannelOperator()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:203:<tr class="memitem:gafc207651653dbde2b3cfd6615695df97"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__channels.html#gafc207651653dbde2b3cfd6615695df97">TT_IsChannelOperator</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN INT32 nChannelID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:527:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__channels.html#gafc207651653dbde2b3cfd6615695df97" title="Check whether user is operator of a channel.">TT_IsChannelOperator</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:919:<h2 class="memtitle"><span class="permalink"><a href="#gafc207651653dbde2b3cfd6615695df97">&#9670;&nbsp;</a></span>TT_IsChannelOperator()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__channels.html:925:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_IsChannelOperator </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:156:  ['tt_5fischanneloperator_820',['TT_IsChannelOperator',['../group__channels.html#gafc207651653dbde2b3cfd6615695df97',1,'TeamTalk.h']]],
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2699:    TEAMTALKDLL_API TTBOOL TT_IsChannelOperator(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\channels.rs:157:        unsafe { ffi::api().TT_IsChannelOperator(self.ptr.0, user_id.0, channel_id.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_MacOS_GetWindow`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2134:<li><a class="el" href="group__desktopshare.html#ga460f5b6dd80177353821f263bad98fbe" title="Enumerate all windows on the desktop. Increment nIndex until the function returns FALSE....">TT_MacOS_GetWindow()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2135:<li><a class="el" href="group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db" title="Get information about a window by passing its handle (CGWindowID).">TT_MacOS_GetWindowFromWindowID()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:529:<li>TT_MacOS_GetWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:532:<li>TT_MacOS_GetWindowFromWindowID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:469:<li>TT_MacOS_GetWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:472:<li>TT_MacOS_GetWindowFromWindowID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:221:<tr class="memitem:ga460f5b6dd80177353821f263bad98fbe"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga460f5b6dd80177353821f263bad98fbe">TT_MacOS_GetWindow</a> (IN INT32 nIndex, OUT <a class="el" href="struct_share_window.html">ShareWindow</a> *lpShareWindow)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:222:<tr class="memdesc:ga460f5b6dd80177353821f263bad98fbe"><td class="mdescLeft">&#160;</td><td class="mdescRight">Enumerate all windows on the desktop. Increment <code>nIndex</code> until the function returns FALSE. Use <a class="el" href="group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db" title="Get information about a window by passing its handle (CGWindowID).">TT_MacOS_GetWindowFromWindowID()</a> to get information about the window, e.g. title, dimensions, etc.  <a href="group__desktopshare.html#ga460f5b6dd80177353821f263bad98fbe">More...</a><br /></td></tr>
... (28 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2384:    TEAMTALKDLL_API TTBOOL TT_MacOS_GetWindow(IN INT32 nIndex,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2388:    TEAMTALKDLL_API TTBOOL TT_MacOS_GetWindowFromWindowID(IN INT64 nWindowID,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_MacOS_GetWindowFromWindowID`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2135:<li><a class="el" href="group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db" title="Get information about a window by passing its handle (CGWindowID).">TT_MacOS_GetWindowFromWindowID()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:532:<li>TT_MacOS_GetWindowFromWindowID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:472:<li>TT_MacOS_GetWindowFromWindowID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:78:    [ "TT_MacOS_GetWindowFromWindowID", "group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:222:<tr class="memdesc:ga460f5b6dd80177353821f263bad98fbe"><td class="mdescLeft">&#160;</td><td class="mdescRight">Enumerate all windows on the desktop. Increment <code>nIndex</code> until the function returns FALSE. Use <a class="el" href="group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db" title="Get information about a window by passing its handle (CGWindowID).">TT_MacOS_GetWindowFromWindowID()</a> to get information about the window, e.g. title, dimensions, etc.  <a href="group__desktopshare.html#ga460f5b6dd80177353821f263bad98fbe">More...</a><br /></td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:224:<tr class="memitem:gaa20a7e5c12eb9ac74758ac1751d972db"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db">TT_MacOS_GetWindowFromWindowID</a> (IN INT64 nWindowID, OUT <a class="el" href="struct_share_window.html">ShareWindow</a> *lpShareWindow)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:311:<li><a class="el" href="group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db" title="Get information about a window by passing its handle (CGWindowID).">TT_MacOS_GetWindowFromWindowID()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:683:<a class="el" href="group__desktopshare.html#gaa20a7e5c12eb9ac74758ac1751d972db" title="Get information about a window by passing its handle (CGWindowID).">TT_MacOS_GetWindowFromWindowID()</a> </dd></dl>
... (11 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2388:    TEAMTALKDLL_API TTBOOL TT_MacOS_GetWindowFromWindowID(IN INT64 nWindowID,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetMixerCount`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:475:<li>TT_Mixer_GetMixerCount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:12:    [ "TT_Mixer_GetMixerCount", "group__mixer.html#ga4cb23a91a578ad1a040a3f8f97f35af6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:538:<li>TT_Mixer_GetMixerCount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:120:<tr class="memitem:ga4cb23a91a578ad1a040a3f8f97f35af6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga4cb23a91a578ad1a040a3f8f97f35af6">TT_Mixer_GetMixerCount</a> (void)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:257:<h2 class="memtitle"><span class="permalink"><a href="#ga4cb23a91a578ad1a040a3f8f97f35af6">&#9670;&nbsp;</a></span>TT_Mixer_GetMixerCount()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:263:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetMixerCount </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:273:<p>The index from 0 to <a class="el" href="group__mixer.html#ga4cb23a91a578ad1a040a3f8f97f35af6" title="Get the number of Windows Mixers available.">TT_Mixer_GetMixerCount()</a>-1 should be passed to the TT_Mixer_* functions. </p><dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mixer.html#gaed3858ea80ef0842fc3a4fe9fb2492d7" title="Get the name of a Windows Mixer based on its name.">TT_Mixer_GetMixerName</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:306:    <tr><td class="paramname">nMixerIndex</td><td>The index of the mixer. Ranging from 0 to <a class="el" href="group__mixer.html#ga4cb23a91a578ad1a040a3f8f97f35af6" title="Get the number of Windows Mixers available.">TT_Mixer_GetMixerCount()</a>-1. </td></tr>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2924:    TEAMTALKDLL_API INT32 TT_Mixer_GetMixerCount(void);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:15:        unsafe { ffi::api().TT_Mixer_GetMixerCount() }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetMixerName`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:478:<li>TT_Mixer_GetMixerName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:13:    [ "TT_Mixer_GetMixerName", "group__mixer.html#gaed3858ea80ef0842fc3a4fe9fb2492d7", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:123:<tr class="memitem:gaed3858ea80ef0842fc3a4fe9fb2492d7"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gaed3858ea80ef0842fc3a4fe9fb2492d7">TT_Mixer_GetMixerName</a> (IN INT32 nMixerIndex, OUT <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szMixerName[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:273:<p>The index from 0 to <a class="el" href="group__mixer.html#ga4cb23a91a578ad1a040a3f8f97f35af6" title="Get the number of Windows Mixers available.">TT_Mixer_GetMixerCount()</a>-1 should be passed to the TT_Mixer_* functions. </p><dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mixer.html#gaed3858ea80ef0842fc3a4fe9fb2492d7" title="Get the name of a Windows Mixer based on its name.">TT_Mixer_GetMixerName</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:278:<h2 class="memtitle"><span class="permalink"><a href="#gaed3858ea80ef0842fc3a4fe9fb2492d7">&#9670;&nbsp;</a></span>TT_Mixer_GetMixerName()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:284:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_GetMixerName </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:541:<li>TT_Mixer_GetMixerName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:163:  ['tt_5fmixer_5fgetmixername_827',['TT_Mixer_GetMixerName',['../group__mixer.html#gaed3858ea80ef0842fc3a4fe9fb2492d7',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2927:    TEAMTALKDLL_API TTBOOL TT_Mixer_GetMixerName(IN INT32 nMixerIndex,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:22:            if ffi::api().TT_Mixer_GetMixerName(index, buf.as_mut_ptr()) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInBoost`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:481:<li>TT_Mixer_GetWaveInBoost()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:544:<li>TT_Mixer_GetWaveInBoost()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:25:    [ "TT_Mixer_GetWaveInBoost", "group__mixer.html#ga8299f27b19cfbed3d37a81b713fa07b6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:159:<tr class="memitem:ga8299f27b19cfbed3d37a81b713fa07b6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga8299f27b19cfbed3d37a81b713fa07b6">TT_Mixer_GetWaveInBoost</a> (IN INT32 nWaveDeviceID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:751:<h2 class="memtitle"><span class="permalink"><a href="#ga8299f27b19cfbed3d37a81b713fa07b6">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInBoost()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:757:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetWaveInBoost </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:164:  ['tt_5fmixer_5fgetwaveinboost_828',['TT_Mixer_GetWaveInBoost',['../group__mixer.html#ga8299f27b19cfbed3d37a81b713fa07b6',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h_source.html:2010:<div class="line"><a name="l08066"></a><span class="lineno"><a class="line" href="group__mixer.html#ga8299f27b19cfbed3d37a81b713fa07b6"> 8066</a></span>&#160;    <a class="code" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 <a class="code" href="group__mixer.html#ga8299f27b19cfbed3d37a81b713fa07b6">TT_Mixer_GetWaveInBoost</a>(IN INT32 nWaveDeviceID);</div>
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2977:    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInBoost(IN INT32 nWaveDeviceID);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:91:        unsafe { ffi::api().TT_Mixer_GetWaveInBoost(wave_id) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInControlCount`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:484:<li>TT_Mixer_GetWaveInControlCount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:547:<li>TT_Mixer_GetWaveInControlCount()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:28:    [ "TT_Mixer_GetWaveInControlCount", "group__mixer.html#ga9e5bf1584ed8db8719424201e27eb728", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:168:<tr class="memitem:ga9e5bf1584ed8db8719424201e27eb728"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga9e5bf1584ed8db8719424201e27eb728">TT_Mixer_GetWaveInControlCount</a> (IN INT32 nWaveDeviceID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:199:<p>Wave-In devices which are not in the enum-structure can be accessed by <a class="el" href="group__mixer.html#ga9e5bf1584ed8db8719424201e27eb728" title="Get the number of Windows Mixer Wave-In devices.">TT_Mixer_GetWaveInControlCount</a> which allows the user to query selection based on an index.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:229:<p>Wave-In devices which are not in the enum-structure can be accessed by <a class="el" href="group__mixer.html#ga9e5bf1584ed8db8719424201e27eb728" title="Get the number of Windows Mixer Wave-In devices.">TT_Mixer_GetWaveInControlCount</a> which allows the user to query selection based on an index.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:842:<h2 class="memtitle"><span class="permalink"><a href="#ga9e5bf1584ed8db8719424201e27eb728">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInControlCount()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:848:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetWaveInControlCount </td>
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2987:    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInControlCount(IN INT32 nWaveDeviceID);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:96:        unsafe { ffi::api().TT_Mixer_GetWaveInControlCount(wave_id) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInControlName`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:487:<li>TT_Mixer_GetWaveInControlName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:550:<li>TT_Mixer_GetWaveInControlName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:29:    [ "TT_Mixer_GetWaveInControlName", "group__mixer.html#ga06f9e12a3ddf0a82cbe0505173b5d405", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:171:<tr class="memitem:ga06f9e12a3ddf0a82cbe0505173b5d405"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga06f9e12a3ddf0a82cbe0505173b5d405">TT_Mixer_GetWaveInControlName</a> (IN INT32 nWaveDeviceID, IN INT32 nControlIndex, OUT <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szDeviceName[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:209:<a class="el" href="group__mixer.html#ga06f9e12a3ddf0a82cbe0505173b5d405" title="Get the name of the Wave-In device with the specified index.">TT_Mixer_GetWaveInControlName</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:239:<a class="el" href="group__mixer.html#ga06f9e12a3ddf0a82cbe0505173b5d405" title="Get the name of the Wave-In device with the specified index.">TT_Mixer_GetWaveInControlName</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:865:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mixer.html#ga06f9e12a3ddf0a82cbe0505173b5d405" title="Get the name of the Wave-In device with the specified index.">TT_Mixer_GetWaveInControlName</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:874:<h2 class="memtitle"><span class="permalink"><a href="#ga06f9e12a3ddf0a82cbe0505173b5d405">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInControlName()</h2>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2990:    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveInControlName(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:103:            if ffi::api().TT_Mixer_GetWaveInControlName(wave_id, index, buf.as_mut_ptr()) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInControlSelected`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:490:<li>TT_Mixer_GetWaveInControlSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:553:<li>TT_Mixer_GetWaveInControlSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:177:<tr class="memitem:gad2a908ae2de592613019dd359b8e0a94"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gad2a908ae2de592613019dd359b8e0a94">TT_Mixer_GetWaveInControlSelected</a> (IN INT32 nWaveDeviceID, IN INT32 nControlIndex)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:869:<a class="el" href="group__mixer.html#gad2a908ae2de592613019dd359b8e0a94" title="Get the selected state of a Wave-In device in the Windows Mixer.">TT_Mixer_GetWaveInControlSelected</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:957:<h2 class="memtitle"><span class="permalink"><a href="#gad2a908ae2de592613019dd359b8e0a94">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInControlSelected()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:963:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_GetWaveInControlSelected </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:31:    [ "TT_Mixer_GetWaveInControlSelected", "group__mixer.html#gad2a908ae2de592613019dd359b8e0a94", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:167:  ['tt_5fmixer_5fgetwaveincontrolselected_831',['TT_Mixer_GetWaveInControlSelected',['../group__mixer.html#gad2a908ae2de592613019dd359b8e0a94',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2999:    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveInControlSelected(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:118:        unsafe { ffi::api().TT_Mixer_GetWaveInControlSelected(wave_id, index) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInMute`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:493:<li>TT_Mixer_GetWaveInMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:556:<li>TT_Mixer_GetWaveInMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:165:<tr class="memitem:ga2fe1782052b6cedc8ecfb4c17ca466e2"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga2fe1782052b6cedc8ecfb4c17ca466e2">TT_Mixer_GetWaveInMute</a> (IN INT32 nWaveDeviceID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:815:<h2 class="memtitle"><span class="permalink"><a href="#ga2fe1782052b6cedc8ecfb4c17ca466e2">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInMute()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:821:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetWaveInMute </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:27:    [ "TT_Mixer_GetWaveInMute", "group__mixer.html#ga2fe1782052b6cedc8ecfb4c17ca466e2", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:168:  ['tt_5fmixer_5fgetwaveinmute_832',['TT_Mixer_GetWaveInMute',['../group__mixer.html#ga2fe1782052b6cedc8ecfb4c17ca466e2',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.js:741:    [ "TT_Mixer_GetWaveInMute", "group__mixer.html#ga2fe1782052b6cedc8ecfb4c17ca466e2", null ],
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2984:    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInMute(IN INT32 nWaveDeviceID);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:57:        unsafe { ffi::api().TT_Mixer_GetWaveInMute(wave_id) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInName`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:559:<li>TT_Mixer_GetWaveInName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:496:<li>TT_Mixer_GetWaveInName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:126:<tr class="memitem:ga03562630b7e2d8487df8d2057b8c67a1"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga03562630b7e2d8487df8d2057b8c67a1">TT_Mixer_GetWaveInName</a> (IN INT32 nWaveDeviceID, OUT <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szMixerName[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:315:<h2 class="memtitle"><span class="permalink"><a href="#ga03562630b7e2d8487df8d2057b8c67a1">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInName()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:321:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_GetWaveInName </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:14:    [ "TT_Mixer_GetWaveInName", "group__mixer.html#ga03562630b7e2d8487df8d2057b8c67a1", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:169:  ['tt_5fmixer_5fgetwaveinname_833',['TT_Mixer_GetWaveInName',['../group__mixer.html#ga03562630b7e2d8487df8d2057b8c67a1',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\struct_sound_device.html:234:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mixer.html#ga03562630b7e2d8487df8d2057b8c67a1" title="Get the name of the mixer associated with a wave-in device.">TT_Mixer_GetWaveInName</a> </dd>
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2931:    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveInName(IN INT32 nWaveDeviceID,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:64:            if ffi::api().TT_Mixer_GetWaveInName(wave_id, buf.as_mut_ptr()) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInSelected`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:499:<li>TT_Mixer_GetWaveInSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:562:<li>TT_Mixer_GetWaveInSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:147:<tr class="memitem:gaf227c34dd23a49688ea682c0fb5ca83a"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gaf227c34dd23a49688ea682c0fb5ca83a">TT_Mixer_GetWaveInSelected</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:594:<h2 class="memtitle"><span class="permalink"><a href="#gaf227c34dd23a49688ea682c0fb5ca83a">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInSelected()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:600:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetWaveInSelected </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:21:    [ "TT_Mixer_GetWaveInSelected", "group__mixer.html#gaf227c34dd23a49688ea682c0fb5ca83a", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:170:  ['tt_5fmixer_5fgetwaveinselected_834',['TT_Mixer_GetWaveInSelected',['../group__mixer.html#gaf227c34dd23a49688ea682c0fb5ca83a',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1728:<tr class="memitem:gaf227c34dd23a49688ea682c0fb5ca83a"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gaf227c34dd23a49688ea682c0fb5ca83a">TT_Mixer_GetWaveInSelected</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl)</td></tr>
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2961:    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInSelected(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveInVolume`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:502:<li>TT_Mixer_GetWaveInVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:565:<li>TT_Mixer_GetWaveInVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:153:<tr class="memitem:gac5e7849a8fbaf8d07820ee01ae1e33a5"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gac5e7849a8fbaf8d07820ee01ae1e33a5">TT_Mixer_GetWaveInVolume</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:676:<h2 class="memtitle"><span class="permalink"><a href="#gac5e7849a8fbaf8d07820ee01ae1e33a5">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveInVolume()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:682:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetWaveInVolume </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:23:    [ "TT_Mixer_GetWaveInVolume", "group__mixer.html#gac5e7849a8fbaf8d07820ee01ae1e33a5", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:171:  ['tt_5fmixer_5fgetwaveinvolume_835',['TT_Mixer_GetWaveInVolume',['../group__mixer.html#gac5e7849a8fbaf8d07820ee01ae1e33a5',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.js:737:    [ "TT_Mixer_GetWaveInVolume", "group__mixer.html#gac5e7849a8fbaf8d07820ee01ae1e33a5", null ],
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2970:    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInVolume(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveOutMute`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:505:<li>TT_Mixer_GetWaveOutMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:568:<li>TT_Mixer_GetWaveOutMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:17:    [ "TT_Mixer_GetWaveOutMute", "group__mixer.html#ga99b526b97c2c2d15a6183b28e53747c7", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:135:<tr class="memitem:ga99b526b97c2c2d15a6183b28e53747c7"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga99b526b97c2c2d15a6183b28e53747c7">TT_Mixer_GetWaveOutMute</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:431:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mixer.html#ga99b526b97c2c2d15a6183b28e53747c7" title="Get the mute state of a Windows Mixer Wave-Out device from the &#39;enum&#39; of devices.">TT_Mixer_GetWaveOutMute</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:436:<h2 class="memtitle"><span class="permalink"><a href="#ga99b526b97c2c2d15a6183b28e53747c7">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveOutMute()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:442:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetWaveOutMute </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:172:  ['tt_5fmixer_5fgetwaveoutmute_836',['TT_Mixer_GetWaveOutMute',['../group__mixer.html#ga99b526b97c2c2d15a6183b28e53747c7',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2944:    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveOutMute(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:37:        unsafe { ffi::api().TT_Mixer_GetWaveOutMute(wave_id, control) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveOutName`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:508:<li>TT_Mixer_GetWaveOutName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:571:<li>TT_Mixer_GetWaveOutName()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:129:<tr class="memitem:gaad2937273ebe61597e9d0e54ee29b7b7"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gaad2937273ebe61597e9d0e54ee29b7b7">TT_Mixer_GetWaveOutName</a> (IN INT32 nWaveDeviceID, OUT <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szMixerName[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:353:<h2 class="memtitle"><span class="permalink"><a href="#gaad2937273ebe61597e9d0e54ee29b7b7">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveOutName()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:359:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_GetWaveOutName </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:15:    [ "TT_Mixer_GetWaveOutName", "group__mixer.html#gaad2937273ebe61597e9d0e54ee29b7b7", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:173:  ['tt_5fmixer_5fgetwaveoutname_837',['TT_Mixer_GetWaveOutName',['../group__mixer.html#gaad2937273ebe61597e9d0e54ee29b7b7',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\struct_sound_device.html:236:<a class="el" href="group__mixer.html#gaad2937273ebe61597e9d0e54ee29b7b7" title="Get the name of the mixer associated with a wave-out device.">TT_Mixer_GetWaveOutName</a> </dd>
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2935:    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveOutName(IN INT32 nWaveDeviceID,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:76:            if ffi::api().TT_Mixer_GetWaveOutName(wave_id, buf.as_mut_ptr()) == 1 {
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_GetWaveOutVolume`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:511:<li>TT_Mixer_GetWaveOutVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:574:<li>TT_Mixer_GetWaveOutVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:141:<tr class="memitem:ga9d92c10d4e04c194a2fbeb6276768e91"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga9d92c10d4e04c194a2fbeb6276768e91">TT_Mixer_GetWaveOutVolume</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:519:<h2 class="memtitle"><span class="permalink"><a href="#ga9d92c10d4e04c194a2fbeb6276768e91">&#9670;&nbsp;</a></span>TT_Mixer_GetWaveOutVolume()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:525:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_Mixer_GetWaveOutVolume </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:19:    [ "TT_Mixer_GetWaveOutVolume", "group__mixer.html#ga9d92c10d4e04c194a2fbeb6276768e91", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:174:  ['tt_5fmixer_5fgetwaveoutvolume_838',['TT_Mixer_GetWaveOutVolume',['../group__mixer.html#ga9d92c10d4e04c194a2fbeb6276768e91',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\functions_0.js:144:  ['tt_5fmixer_5fgetwaveoutvolume_1296',['TT_Mixer_GetWaveOutVolume',['../group__mixer.html#ga9d92c10d4e04c194a2fbeb6276768e91',1,'TeamTalk.h']]],
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2953:    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveOutVolume(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:47:        unsafe { ffi::api().TT_Mixer_GetWaveOutVolume(wave_id, control) }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_SetWaveInBoost`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:514:<li>TT_Mixer_SetWaveInBoost()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:577:<li>TT_Mixer_SetWaveInBoost()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:24:    [ "TT_Mixer_SetWaveInBoost", "group__mixer.html#gaeaa75b6343d78401732862c907692c57", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:156:<tr class="memitem:gaeaa75b6343d78401732862c907692c57"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gaeaa75b6343d78401732862c907692c57">TT_Mixer_SetWaveInBoost</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEnable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:714:<h2 class="memtitle"><span class="permalink"><a href="#gaeaa75b6343d78401732862c907692c57">&#9670;&nbsp;</a></span>TT_Mixer_SetWaveInBoost()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:720:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_SetWaveInBoost </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:175:  ['tt_5fmixer_5fsetwaveinboost_839',['TT_Mixer_SetWaveInBoost',['../group__mixer.html#gaeaa75b6343d78401732862c907692c57',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.html:1737:<tr class="memitem:gaeaa75b6343d78401732862c907692c57"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gaeaa75b6343d78401732862c907692c57">TT_Mixer_SetWaveInBoost</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEnable)</td></tr>
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2974:    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInBoost(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:86:        unsafe { ffi::api().TT_Mixer_SetWaveInBoost(wave_id, enable as i32) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_SetWaveInControlSelected`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:517:<li>TT_Mixer_SetWaveInControlSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:580:<li>TT_Mixer_SetWaveInControlSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:30:    [ "TT_Mixer_SetWaveInControlSelected", "group__mixer.html#gab5c5d2d0ea66177cccf71f9ed7f78712", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:174:<tr class="memitem:gab5c5d2d0ea66177cccf71f9ed7f78712"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gab5c5d2d0ea66177cccf71f9ed7f78712">TT_Mixer_SetWaveInControlSelected</a> (IN INT32 nWaveDeviceID, IN INT32 nControlIndex)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:211:<a class="el" href="group__mixer.html#gab5c5d2d0ea66177cccf71f9ed7f78712" title="Set the selected state of a Wave-In device in the Windows Mixer.">TT_Mixer_SetWaveInControlSelected</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:241:<a class="el" href="group__mixer.html#gab5c5d2d0ea66177cccf71f9ed7f78712" title="Set the selected state of a Wave-In device in the Windows Mixer.">TT_Mixer_SetWaveInControlSelected</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:867:<a class="el" href="group__mixer.html#gab5c5d2d0ea66177cccf71f9ed7f78712" title="Set the selected state of a Wave-In device in the Windows Mixer.">TT_Mixer_SetWaveInControlSelected</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:919:<h2 class="memtitle"><span class="permalink"><a href="#gab5c5d2d0ea66177cccf71f9ed7f78712">&#9670;&nbsp;</a></span>TT_Mixer_SetWaveInControlSelected()</h2>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2995:    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInControlSelected(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:113:        unsafe { ffi::api().TT_Mixer_SetWaveInControlSelected(wave_id, index) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_SetWaveInMute`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:520:<li>TT_Mixer_SetWaveInMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:583:<li>TT_Mixer_SetWaveInMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:26:    [ "TT_Mixer_SetWaveInMute", "group__mixer.html#ga3ef429e61f6cb85b5ff065354455c5aa", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:162:<tr class="memitem:ga3ef429e61f6cb85b5ff065354455c5aa"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga3ef429e61f6cb85b5ff065354455c5aa">TT_Mixer_SetWaveInMute</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bEnable)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:778:<h2 class="memtitle"><span class="permalink"><a href="#ga3ef429e61f6cb85b5ff065354455c5aa">&#9670;&nbsp;</a></span>TT_Mixer_SetWaveInMute()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:784:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_SetWaveInMute </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:177:  ['tt_5fmixer_5fsetwaveinmute_841',['TT_Mixer_SetWaveInMute',['../group__mixer.html#ga3ef429e61f6cb85b5ff065354455c5aa',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\_team_talk_8h.js:740:    [ "TT_Mixer_SetWaveInMute", "group__mixer.html#ga3ef429e61f6cb85b5ff065354455c5aa", null ],
... (4 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2980:    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInMute(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:52:        unsafe { ffi::api().TT_Mixer_SetWaveInMute(wave_id, mute as i32) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_SetWaveInSelected`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:523:<li>TT_Mixer_SetWaveInSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:586:<li>TT_Mixer_SetWaveInSelected()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:20:    [ "TT_Mixer_SetWaveInSelected", "group__mixer.html#gae47642be7a1a3b7d308f2468f6fbed69", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:144:<tr class="memitem:gae47642be7a1a3b7d308f2468f6fbed69"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gae47642be7a1a3b7d308f2468f6fbed69">TT_Mixer_SetWaveInSelected</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:205:<a class="el" href="group__mixer.html#gae47642be7a1a3b7d308f2468f6fbed69" title="Set the selected state of a Windows Mixer Wave-In device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveInSelected</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:235:<a class="el" href="group__mixer.html#gae47642be7a1a3b7d308f2468f6fbed69" title="Set the selected state of a Windows Mixer Wave-In device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveInSelected</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:557:<h2 class="memtitle"><span class="permalink"><a href="#gae47642be7a1a3b7d308f2468f6fbed69">&#9670;&nbsp;</a></span>TT_Mixer_SetWaveInSelected()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:563:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_SetWaveInSelected </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2957:    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInSelected(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_SetWaveInVolume`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:526:<li>TT_Mixer_SetWaveInVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:589:<li>TT_Mixer_SetWaveInVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:150:<tr class="memitem:gadae2f1373098fd1ed727eeb814d39cb4"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#gadae2f1373098fd1ed727eeb814d39cb4">TT_Mixer_SetWaveInVolume</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl, IN INT32 nVolume)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:207:<a class="el" href="group__mixer.html#gadae2f1373098fd1ed727eeb814d39cb4" title="Set the volume of a Windows Mixer Wave-In device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveInVolume</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:237:<a class="el" href="group__mixer.html#gadae2f1373098fd1ed727eeb814d39cb4" title="Set the volume of a Windows Mixer Wave-In device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveInVolume</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:632:<h2 class="memtitle"><span class="permalink"><a href="#gadae2f1373098fd1ed727eeb814d39cb4">&#9670;&nbsp;</a></span>TT_Mixer_SetWaveInVolume()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:638:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_SetWaveInVolume </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:22:    [ "TT_Mixer_SetWaveInVolume", "group__mixer.html#gadae2f1373098fd1ed727eeb814d39cb4", null ],
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2965:    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInVolume(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_SetWaveOutMute`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:529:<li>TT_Mixer_SetWaveOutMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:592:<li>TT_Mixer_SetWaveOutMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:16:    [ "TT_Mixer_SetWaveOutMute", "group__mixer.html#ga4db0690621e16069eb9f59ae0aec5818", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:132:<tr class="memitem:ga4db0690621e16069eb9f59ae0aec5818"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga4db0690621e16069eb9f59ae0aec5818">TT_Mixer_SetWaveOutMute</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bMute)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:201:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mixer.html#ga4db0690621e16069eb9f59ae0aec5818" title="Mute or unmute a Windows Mixer Wave-Out device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveOutMute</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:231:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mixer.html#ga4db0690621e16069eb9f59ae0aec5818" title="Mute or unmute a Windows Mixer Wave-Out device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveOutMute</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:391:<h2 class="memtitle"><span class="permalink"><a href="#ga4db0690621e16069eb9f59ae0aec5818">&#9670;&nbsp;</a></span>TT_Mixer_SetWaveOutMute()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:397:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_SetWaveOutMute </td>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2939:    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveOutMute(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:32:        unsafe { ffi::api().TT_Mixer_SetWaveOutMute(wave_id, control, mute as i32) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Mixer_SetWaveOutVolume`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:532:<li>TT_Mixer_SetWaveOutVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:595:<li>TT_Mixer_SetWaveOutVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.js:18:    [ "TT_Mixer_SetWaveOutVolume", "group__mixer.html#ga336d909663ba4302ae0fb13660120108", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:138:<tr class="memitem:ga336d909663ba4302ae0fb13660120108"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mixer.html#ga336d909663ba4302ae0fb13660120108">TT_Mixer_SetWaveOutVolume</a> (IN INT32 nWaveDeviceID, IN <a class="el" href="group__mixer.html#ga5a1ab025ea38742d18797adf727873aa">MixerControl</a> nControl, IN INT32 nVolume)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:203:<a class="el" href="group__mixer.html#ga336d909663ba4302ae0fb13660120108" title="Set the volume of a Windows Mixer Wave-Out device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveOutVolume</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:233:<a class="el" href="group__mixer.html#ga336d909663ba4302ae0fb13660120108" title="Set the volume of a Windows Mixer Wave-Out device from the &#39;enum&#39; of devices.">TT_Mixer_SetWaveOutVolume</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:475:<h2 class="memtitle"><span class="permalink"><a href="#ga336d909663ba4302ae0fb13660120108">&#9670;&nbsp;</a></span>TT_Mixer_SetWaveOutVolume()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mixer.html:481:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Mixer_SetWaveOutVolume </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2948:    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveOutVolume(IN INT32 nWaveDeviceID, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\mixer.rs:42:        unsafe { ffi::api().TT_Mixer_SetWaveOutVolume(wave_id, control, vol) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_PaintDesktopWindow`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2119:<li><a class="el" href="group__desktopshare.html#gad20b74ee5e74833a08740abe2f6a6651" title="Paint user&#39;s desktop window using a Windows&#39; DC (device context).">TT_PaintDesktopWindow()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2120:<li><a class="el" href="group__desktopshare.html#gaacccb83d44789feffa68b853da87f5f3" title="Paint user&#39;s desktop window using a Windows&#39; DC (device context).">TT_PaintDesktopWindowEx()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:535:<li>TT_PaintDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:538:<li>TT_PaintDesktopWindowEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:601:<li>TT_PaintDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:604:<li>TT_PaintDesktopWindowEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:75:    [ "TT_PaintDesktopWindow", "group__desktopshare.html#gad20b74ee5e74833a08740abe2f6a6651", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:76:    [ "TT_PaintDesktopWindowEx", "group__desktopshare.html#gaacccb83d44789feffa68b853da87f5f3", null ],
... (20 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2340:    TEAMTALKDLL_API TTBOOL TT_PaintDesktopWindow(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2349:    TEAMTALKDLL_API TTBOOL TT_PaintDesktopWindowEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_PaintDesktopWindowEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2120:<li><a class="el" href="group__desktopshare.html#gaacccb83d44789feffa68b853da87f5f3" title="Paint user&#39;s desktop window using a Windows&#39; DC (device context).">TT_PaintDesktopWindowEx()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:538:<li>TT_PaintDesktopWindowEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:604:<li>TT_PaintDesktopWindowEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:218:<tr class="memitem:gaacccb83d44789feffa68b853da87f5f3"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#gaacccb83d44789feffa68b853da87f5f3">TT_PaintDesktopWindowEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN HDC hDC, IN INT32 XDest, IN INT32 YDest, IN INT32 nDestWidth, IN INT32 nDestHeight, IN INT32 XSrc, IN INT32 YSrc, IN INT32 nSrcWidth, IN INT32 nSrcHeight)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1150:<p>Same as calling <a class="el" href="group__desktopshare.html#gaacccb83d44789feffa68b853da87f5f3" title="Paint user&#39;s desktop window using a Windows&#39; DC (device context).">TT_PaintDesktopWindowEx()</a> like this:</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1151:<pre class="fragment">TT_PaintDesktopWindowEx(lpTTInstance, nUserID, hDC, 
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1160:<h2 class="memtitle"><span class="permalink"><a href="#gaacccb83d44789feffa68b853da87f5f3">&#9670;&nbsp;</a></span>TT_PaintDesktopWindowEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1166:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_PaintDesktopWindowEx </td>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2349:    TEAMTALKDLL_API TTBOOL TT_PaintDesktopWindowEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_PaintVideoFrame`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2118:<li><a class="el" href="group__videocapture.html#ga8d0cbbeaa431f3e4c7608b713e300b08" title="Paint user&#39;s video frame using a Windows&#39; DC (device context).">TT_PaintVideoFrameEx()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:541:<li>TT_PaintVideoFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:544:<li>TT_PaintVideoFrameEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:607:<li>TT_PaintVideoFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:610:<li>TT_PaintVideoFrameEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:152:<tr class="memitem:gadb6d6a181d979f400fdf84e6507daed1"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__videocapture.html#gadb6d6a181d979f400fdf84e6507daed1">TT_PaintVideoFrame</a> (IN HDC hDC, IN INT32 XDest, IN INT32 YDest, IN INT32 nDestWidth, IN INT32 nDestHeight, IN <a class="el" href="struct_video_frame.html">VideoFrame</a> *lpVideoFrame)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:155:<tr class="memitem:ga8d0cbbeaa431f3e4c7608b713e300b08"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__videocapture.html#ga8d0cbbeaa431f3e4c7608b713e300b08">TT_PaintVideoFrameEx</a> (IN HDC hDC, IN INT32 XDest, IN INT32 YDest, IN INT32 nDestWidth, IN INT32 nDestHeight, IN INT32 XSrc, IN INT32 YSrc, IN INT32 nSrcWidth, IN INT32 nSrcHeight, IN <a class="el" href="struct_video_frame.html">VideoFrame</a> *lpVideoFrame)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:179:<p>When a video frame becomes available the event <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a02d0b929382287a55fc6f54d607eb261" title="A new video frame from a video capture device was received from a user.">CLIENTEVENT_USER_VIDEOCAPTURE</a> is posted to the application and <a class="el" href="group__videocapture.html#ga21c3d6e6a8cb56b5eef7695e42032990" title="Extract a user&#39;s video capture frame for display.">TT_AcquireUserVideoCaptureFrame</a> can be used to extract the RGB32 image. On Windows it's also possible to call <a class="el" href="group__videocapture.html#gadb6d6a181d979f400fdf84e6507daed1" title="Paint user&#39;s video frame using a Windows&#39; DC (device context).">TT_PaintVideoFrame</a> to make the client instance paint on a HWND by getting its HDC, otherwise use the <a class="el" href="struct_video_frame.html" title="A RGB32 image where the pixels can be accessed directly in an allocated frameBuffer.">VideoFrame</a>'s frame buffer pointer to access the bitmap data. </p>
... (20 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2210:    TEAMTALKDLL_API TTBOOL TT_PaintVideoFrame(IN HDC hDC,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2218:    TEAMTALKDLL_API TTBOOL TT_PaintVideoFrameEx(IN HDC hDC,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_PaintVideoFrameEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2118:<li><a class="el" href="group__videocapture.html#ga8d0cbbeaa431f3e4c7608b713e300b08" title="Paint user&#39;s video frame using a Windows&#39; DC (device context).">TT_PaintVideoFrameEx()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:544:<li>TT_PaintVideoFrameEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:610:<li>TT_PaintVideoFrameEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.js:40:    [ "TT_PaintVideoFrameEx", "group__videocapture.html#ga8d0cbbeaa431f3e4c7608b713e300b08", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:155:<tr class="memitem:ga8d0cbbeaa431f3e4c7608b713e300b08"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__videocapture.html#ga8d0cbbeaa431f3e4c7608b713e300b08">TT_PaintVideoFrameEx</a> (IN HDC hDC, IN INT32 XDest, IN INT32 YDest, IN INT32 nDestWidth, IN INT32 nDestHeight, IN INT32 XSrc, IN INT32 YSrc, IN INT32 nSrcWidth, IN INT32 nSrcHeight, IN <a class="el" href="struct_video_frame.html">VideoFrame</a> *lpVideoFrame)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:461:<p>Same as calling <a class="el" href="group__videocapture.html#ga8d0cbbeaa431f3e4c7608b713e300b08" title="Paint user&#39;s video frame using a Windows&#39; DC (device context).">TT_PaintVideoFrameEx()</a> like this:</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:462:<pre class="fragment">TT_PaintVideoFrameEx(lpTTInstance, nUserID, hDC, 
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:471:<h2 class="memtitle"><span class="permalink"><a href="#ga8d0cbbeaa431f3e4c7608b713e300b08">&#9670;&nbsp;</a></span>TT_PaintVideoFrameEx()</h2>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2218:    TEAMTALKDLL_API TTBOOL TT_PaintVideoFrameEx(IN HDC hDC,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Palette_GetColorTable`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2129:<li><a class="el" href="group__desktopshare.html#ga21388f108a7fc0dd9aa2d796595b9f7c" title="Get RGB values of the palette for the bitmap format.">TT_Palette_GetColorTable()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:547:<li>TT_Palette_GetColorTable()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:613:<li>TT_Palette_GetColorTable()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:69:    [ "TT_Palette_GetColorTable", "group__desktopshare.html#ga21388f108a7fc0dd9aa2d796595b9f7c", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:197:<tr class="memitem:ga21388f108a7fc0dd9aa2d796595b9f7c"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> unsigned char *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga21388f108a7fc0dd9aa2d796595b9f7c">TT_Palette_GetColorTable</a> (IN <a class="el" href="group__desktopshare.html#ga23d1a7c7cf0f6da45ca389904e644d55">BitmapFormat</a> nBmpPalette, IN INT32 nIndex)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:721:<tr><td class="fieldname"><a id="gga23d1a7c7cf0f6da45ca389904e644d55a64c7724d85f4127d16416be4e90fee4a"></a>BMP_RGB8_PALETTE&#160;</td><td class="fielddoc"><p>The bitmap is a 256-colored bitmap requiring a palette. The default 256 colored palette is the Netscape browser-safe palette. Use <a class="el" href="group__desktopshare.html#ga21388f108a7fc0dd9aa2d796595b9f7c" title="Get RGB values of the palette for the bitmap format.">TT_Palette_GetColorTable()</a> to access or change the palette. The maximum size of a 8-bit bitmap is 4095 blocks of 120 by 34 pixels. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:901:<h2 class="memtitle"><span class="permalink"><a href="#ga21388f108a7fc0dd9aa2d796595b9f7c">&#9670;&nbsp;</a></span>TT_Palette_GetColorTable()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:907:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> unsigned char* TT_Palette_GetColorTable </td>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2298:    TEAMTALKDLL_API unsigned char* TT_Palette_GetColorTable(IN BitmapFormat nBmpPalette,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_PumpMessage`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1059:<li>Added function <a class="el" href="group__initclient.html#ga2eb567a5d3d0284292adc6c03d503e50" title="Cause client instance event thread to schedule an update event.">TT_PumpMessage()</a> for getting latest <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a>-state.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:550:<li>TT_PumpMessage()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:616:<li>TT_PumpMessage()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:162:<tr class="memitem:ga2eb567a5d3d0284292adc6c03d503e50"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__initclient.html#ga2eb567a5d3d0284292adc6c03d503e50">TT_PumpMessage</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, <a class="el" href="group__events.html#ga7c228530d18e96b483502c824c700224">ClientEvent</a> nClientEvent, INT32 nIdentifier)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:509:<h2 class="memtitle"><span class="permalink"><a href="#ga2eb567a5d3d0284292adc6c03d503e50">&#9670;&nbsp;</a></span>TT_PumpMessage()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:515:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_PumpMessage </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:542:<p>Calling <a class="el" href="group__initclient.html#ga2eb567a5d3d0284292adc6c03d503e50" title="Cause client instance event thread to schedule an update event.">TT_PumpMessage()</a> will make the client instance's internal thread queue an update of <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a> so the latest properties of the user can be retrieved from <a class="el" href="group__initclient.html#ga34fe8de6133a101aa70574225d7dcae0" title="Poll for events in the client instance.">TT_GetMessage()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.js:34:    [ "TT_PumpMessage", "group__initclient.html#ga2eb567a5d3d0284292adc6c03d503e50", null ],
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1984:    TEAMTALKDLL_API TTBOOL TT_PumpMessage(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:118:        unsafe { ffi::api().TT_PumpMessage(self.ptr.0, event, id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_QueryMaxPayload`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2091:<p>Some routers don't allow UDP packets over a given size so use <a class="el" href="group__connectivity.html#ga3a61abf7a0b7f51e3b26ef15379aa9dd" title="Query the maximum size of UDP data packets to the user or server.">TT_QueryMaxPayload()</a> after connecting to a server to detect the maximum size for UDP packets. The event <code>WM_TEAMTALK_CON_MAX_PAYLOAD_UPDATED</code> is triggered when the client instance has finished querying.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2128:<li><a class="el" href="group__connectivity.html#ga3a61abf7a0b7f51e3b26ef15379aa9dd" title="Query the maximum size of UDP data packets to the user or server.">TT_QueryMaxPayload()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:553:<li>TT_QueryMaxPayload()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:619:<li>TT_QueryMaxPayload()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:144:<tr class="memitem:ga3a61abf7a0b7f51e3b26ef15379aa9dd"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__connectivity.html#ga3a61abf7a0b7f51e3b26ef15379aa9dd">TT_QueryMaxPayload</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:548:<h2 class="memtitle"><span class="permalink"><a href="#ga3a61abf7a0b7f51e3b26ef15379aa9dd">&#9670;&nbsp;</a></span>TT_QueryMaxPayload()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:554:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_QueryMaxPayload </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.js:54:    [ "TT_QueryMaxPayload", "group__connectivity.html#ga3a61abf7a0b7f51e3b26ef15379aa9dd", null ],
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2461:    TEAMTALKDLL_API TTBOOL TT_QueryMaxPayload(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:107:        unsafe { ffi::api().TT_QueryMaxPayload(self.ptr.0, user_id.0) == 1 }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\server.rs:112:        unsafe { ffi::api().TT_QueryMaxPayload(self.ptr.0, 0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_ReleaseUserAudioBlock`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1935:<li>Use <a class="el" href="group__sounddevices.html#ga48bd8f8fe21b5acc526419fa85ea4907" title="Release the shared memory of an AudioBlock.">TT_ReleaseUserAudioBlock()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2112:<li><a class="el" href="group__sounddevices.html#ga48bd8f8fe21b5acc526419fa85ea4907" title="Release the shared memory of an AudioBlock.">TT_ReleaseUserAudioBlock()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:556:<li>TT_ReleaseUserAudioBlock()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:622:<li>TT_ReleaseUserAudioBlock()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:123:    [ "TT_ReleaseUserAudioBlock", "group__sounddevices.html#ga48bd8f8fe21b5acc526419fa85ea4907", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:348:<tr class="memitem:ga48bd8f8fe21b5acc526419fa85ea4907"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga48bd8f8fe21b5acc526419fa85ea4907">TT_ReleaseUserAudioBlock</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="struct_audio_block.html">AudioBlock</a> *lpAudioBlock)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:699:<a class="el" href="group__sounddevices.html#ga48bd8f8fe21b5acc526419fa85ea4907" title="Release the shared memory of an AudioBlock.">TT_ReleaseUserAudioBlock()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2156:<a class="el" href="group__sounddevices.html#ga48bd8f8fe21b5acc526419fa85ea4907" title="Release the shared memory of an AudioBlock.">TT_ReleaseUserAudioBlock()</a> </dd>
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2815:    TEAMTALKDLL_API TTBOOL TT_ReleaseUserAudioBlock(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:448:        unsafe { ffi::api().TT_ReleaseUserAudioBlock(self.ptr.0, block) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_ReleaseUserDesktopWindow`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1674:<li><a class="el" href="group__desktopshare.html#ga49e1d5bf5b0d4d0435c1de85b7846f09" title="Release memory allocated by the DesktopWindow.">TT_ReleaseUserDesktopWindow()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:559:<li>TT_ReleaseUserDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:625:<li>TT_ReleaseUserDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:84:    [ "TT_ReleaseUserDesktopWindow", "group__desktopshare.html#ga49e1d5bf5b0d4d0435c1de85b7846f09", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:242:<tr class="memitem:ga49e1d5bf5b0d4d0435c1de85b7846f09"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga49e1d5bf5b0d4d0435c1de85b7846f09">TT_ReleaseUserDesktopWindow</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="struct_desktop_window.html">DesktopWindow</a> *lpDesktopWindow)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1514:<p>When the <a class="el" href="struct_desktop_window.html" title="A struct containing the properties of a shared desktop window.">DesktopWindow</a> is no longer needed call <a class="el" href="group__desktopshare.html#ga49e1d5bf5b0d4d0435c1de85b7846f09" title="Release memory allocated by the DesktopWindow.">TT_ReleaseUserDesktopWindow()</a> to release the memory allocated by the client instance.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1577:<h2 class="memtitle"><span class="permalink"><a href="#ga49e1d5bf5b0d4d0435c1de85b7846f09">&#9670;&nbsp;</a></span>TT_ReleaseUserDesktopWindow()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1583:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_ReleaseUserDesktopWindow </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2418:    TEAMTALKDLL_API TTBOOL TT_ReleaseUserDesktopWindow(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\desktop.rs:39:        unsafe { ffi::api().TT_ReleaseUserDesktopWindow(self.ptr.0, window) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_ReleaseUserMediaVideoFrame`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1662:<li><a class="el" href="group__mediastream.html#gaf9a013f71dcd0f0954f2356538cac88a" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserMediaVideoFrame(), so its allocated resou...">TT_ReleaseUserMediaVideoFrame()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:628:<li>TT_ReleaseUserMediaVideoFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:562:<li>TT_ReleaseUserMediaVideoFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.js:62:    [ "TT_ReleaseUserMediaVideoFrame", "group__mediastream.html#gaf9a013f71dcd0f0954f2356538cac88a", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:199:<tr class="memitem:gaf9a013f71dcd0f0954f2356538cac88a"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#gaf9a013f71dcd0f0954f2356538cac88a">TT_ReleaseUserMediaVideoFrame</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="struct_video_frame.html">VideoFrame</a> *lpVideoFrame)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:786:<p>To release the acquired <a class="el" href="struct_video_frame.html" title="A RGB32 image where the pixels can be accessed directly in an allocated frameBuffer.">VideoFrame</a> call <a class="el" href="group__mediastream.html#gaf9a013f71dcd0f0954f2356538cac88a" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserMediaVideoFrame(), so its allocated resou...">TT_ReleaseUserMediaVideoFrame()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:796:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mediastream.html#gaf9a013f71dcd0f0954f2356538cac88a" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserMediaVideoFrame(), so its allocated resou...">TT_ReleaseUserMediaVideoFrame()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:801:<h2 class="memtitle"><span class="permalink"><a href="#gaf9a013f71dcd0f0954f2356538cac88a">&#9670;&nbsp;</a></span>TT_ReleaseUserMediaVideoFrame()</h2>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2283:    TEAMTALKDLL_API TTBOOL TT_ReleaseUserMediaVideoFrame(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:117:        unsafe { ffi::api().TT_ReleaseUserMediaVideoFrame(self.ptr.0, frame) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_ReleaseUserVideoCaptureFrame`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1654:<li><a class="el" href="group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserVideoCaptureFrame(), so its allocated res...">TT_ReleaseUserVideoCaptureFrame()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1835:<li>Use <a class="el" href="group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserVideoCaptureFrame(), so its allocated res...">TT_ReleaseUserVideoCaptureFrame()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:565:<li>TT_ReleaseUserVideoCaptureFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:631:<li>TT_ReleaseUserVideoCaptureFrame()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.js:42:    [ "TT_ReleaseUserVideoCaptureFrame", "group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:161:<tr class="memitem:gadc629ecc77171b18fb6760fd0539716d"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d">TT_ReleaseUserVideoCaptureFrame</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="struct_video_frame.html">VideoFrame</a> *lpVideoFrame)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:248:<p>Use <a class="el" href="group__videocapture.html#ga21c3d6e6a8cb56b5eef7695e42032990" title="Extract a user&#39;s video capture frame for display.">TT_AcquireUserVideoCaptureFrame()</a> to acquire a user's image and remember to call <a class="el" href="group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserVideoCaptureFrame(), so its allocated res...">TT_ReleaseUserVideoCaptureFrame()</a> when the image has been processed so TeamTalk can release its resources. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__videocapture.html:597:<p>To release the acquired <a class="el" href="struct_video_frame.html" title="A RGB32 image where the pixels can be accessed directly in an allocated frameBuffer.">VideoFrame</a> call <a class="el" href="group__videocapture.html#gadc629ecc77171b18fb6760fd0539716d" title="Delete a user&#39;s video frame, acquired through TT_AcquireUserVideoCaptureFrame(), so its allocated res...">TT_ReleaseUserVideoCaptureFrame()</a>.</p>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2235:    TEAMTALKDLL_API TTBOOL TT_ReleaseUserVideoCaptureFrame(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:84:        unsafe { ffi::api().TT_ReleaseUserVideoCaptureFrame(self.ptr.0, frame) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_RestartSoundSystem`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2035:<p><a class="el" href="group__sounddevices.html#ga2cc9699a6b3b735591c6ed4460066488" title="Reinitialize sound system (in order to detect new/removed devices).">TT_RestartSoundSystem()</a> can now be used to shut down the sound systems can rescan for new devices.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2068:<li><a class="el" href="group__sounddevices.html#ga2cc9699a6b3b735591c6ed4460066488" title="Reinitialize sound system (in order to detect new/removed devices).">TT_RestartSoundSystem()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2485:<li><code>TT_RestartSoundSystem</code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:568:<li>TT_RestartSoundSystem()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:634:<li>TT_RestartSoundSystem()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:745:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga2cc9699a6b3b735591c6ed4460066488" title="Reinitialize sound system (in order to detect new/removed devices).">TT_RestartSoundSystem()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:757:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga2cc9699a6b3b735591c6ed4460066488" title="Reinitialize sound system (in order to detect new/removed devices).">TT_RestartSoundSystem()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:770:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga2cc9699a6b3b735591c6ed4460066488" title="Reinitialize sound system (in order to detect new/removed devices).">TT_RestartSoundSystem()</a> </dd>
... (15 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2012:    TEAMTALKDLL_API TTBOOL TT_RestartSoundSystem(void);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:74:        unsafe { ffi::api().TT_RestartSoundSystem() == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:707:<li>Modified macro <a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095" title="Sound device ID for iOS AudioUnit subtype Voice-Processing I/O Unit.">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a> to include <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>. Previously the iOS sound device that does voice preprocessing actually ran in its own shared device. However, with the introduction of <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a> in TeamTalk v5.5 it is simpler for iOS to use the same shared device property as on Android.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:727:<p>Value of <a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095" title="Sound device ID for iOS AudioUnit subtype Voice-Processing I/O Unit.">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a> has been changed to include <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>. Previously the iOS sound device that does voice preprocessing actually ran in its own shared device. However, with the introduction of <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f" title="Flag/bit in nDeviceID telling if the SoundDevice is a shared version of an existing sound device.">TT_SOUNDDEVICE_ID_SHARED_FLAG</a> in TeamTalk v5.5 it is simpler for iOS to use the same shared device property as on Android. Therefore ensure that <a class="el" href="group__sounddevices.html#ga98f79720f72da9cefd5408c40af9053a" title="Initialize the sound input device (for recording audio).">TT_InitSoundInputDevice()</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice()</a> is not called with 1 instead of the value of <a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095" title="Sound device ID for iOS AudioUnit subtype Voice-Processing I/O Unit.">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_defs.html:213:<li>TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:733:<li>TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:124:<tr class="memitem:ga682257c2d0a203795a6e1ed55d550095"><td class="memItemLeft" align="right" valign="top">#define&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga682257c2d0a203795a6e1ed55d550095">TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</a>&#160;&#160;&#160;(1 | <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:435:<h2 class="memtitle"><span class="permalink"><a href="#ga682257c2d0a203795a6e1ed55d550095">&#9670;&nbsp;</a></span>TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:441:          <td class="memname">#define TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO&#160;&#160;&#160;(1 | <a class="el" href="group__sounddevices.html#ga1fbff4ede397a747f99e0c7d213dd59f">TT_SOUNDDEVICE_ID_SHARED_FLAG</a>)</td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:769:   #TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO will be AudioUnit
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:269:#define TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO  (1 | TT_SOUNDDEVICE_ID_SHARED_FLAG)
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SendDesktopCursorPosition`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1730:<li><a class="el" href="group__desktopshare.html#gaad6b062e926d9a9dc04b9cbc496238fb" title="Send the position of mouse cursor to users in the same channel.">TT_SendDesktopCursorPosition()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2124:<li><a class="el" href="group__desktopshare.html#gaad6b062e926d9a9dc04b9cbc496238fb" title="Send the position of mouse cursor to users in the same channel.">TT_SendDesktopCursorPosition()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:571:<li>TT_SendDesktopCursorPosition()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:640:<li>TT_SendDesktopCursorPosition()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:230:<tr class="memitem:gaad6b062e926d9a9dc04b9cbc496238fb"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#gaad6b062e926d9a9dc04b9cbc496238fb">TT_SendDesktopCursorPosition</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN UINT16 nPosX, IN UINT16 nPosY)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:328:<p>It is also possible to share the position of the mouse cursor when sharing a desktop window. Use <a class="el" href="group__desktopshare.html#gaad6b062e926d9a9dc04b9cbc496238fb" title="Send the position of mouse cursor to users in the same channel.">TT_SendDesktopCursorPosition()</a> to transmit the position of the mouse cursor. When the position is received the event <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224a42932551abb7c1ba2296b9b007279597" title="A user has sent the position of the mouse cursor.">CLIENTEVENT_USER_DESKTOPCURSOR</a> is posted to the local client instance with the mouse coordinates.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:869:<a class="el" href="group__desktopshare.html#gaad6b062e926d9a9dc04b9cbc496238fb" title="Send the position of mouse cursor to users in the same channel.">TT_SendDesktopCursorPosition()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1381:<h2 class="memtitle"><span class="permalink"><a href="#gaad6b062e926d9a9dc04b9cbc496238fb">&#9670;&nbsp;</a></span>TT_SendDesktopCursorPosition()</h2>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2399:    TEAMTALKDLL_API TTBOOL TT_SendDesktopCursorPosition(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\desktop.rs:14:        unsafe { ffi::api().TT_SendDesktopCursorPosition(self.ptr.0, x, y) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SendDesktopFromWindowID`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2123:<li><a class="el" href="group__desktopshare.html#gaa8bd576e39966af10de95f955bccfe3b" title="Transmit the specified window in a desktop session.">TT_SendDesktopFromWindowID()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:574:<li>TT_SendDesktopFromWindowID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:643:<li>TT_SendDesktopFromWindowID()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:227:<tr class="memitem:gaa8bd576e39966af10de95f955bccfe3b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#gaa8bd576e39966af10de95f955bccfe3b">TT_SendDesktopFromWindowID</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT64 nWindowID, IN <a class="el" href="group__desktopshare.html#ga23d1a7c7cf0f6da45ca389904e644d55">BitmapFormat</a> nBitmapFormat, IN <a class="el" href="group__desktopshare.html#ga774e2ca94287587e141c55736e38efe3">DesktopProtocol</a> nDesktopProtocol)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:318:<li><a class="el" href="group__desktopshare.html#gaa8bd576e39966af10de95f955bccfe3b" title="Transmit the specified window in a desktop session.">TT_SendDesktopFromWindowID()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1328:<h2 class="memtitle"><span class="permalink"><a href="#gaa8bd576e39966af10de95f955bccfe3b">&#9670;&nbsp;</a></span>TT_SendDesktopFromWindowID()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1334:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_SendDesktopFromWindowID </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:79:    [ "TT_SendDesktopFromWindowID", "group__desktopshare.html#gaa8bd576e39966af10de95f955bccfe3b", null ],
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2392:    TEAMTALKDLL_API INT32 TT_SendDesktopFromWindowID(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SendDesktopInput`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1974:<li><a class="el" href="group__desktopshare.html#ga00a5bf688d7556e8fe5bfec1b9608a17" title="Send a mouse or keyboard event to a shared desktop window.">TT_SendDesktopInput</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:577:<li>TT_SendDesktopInput()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:646:<li>TT_SendDesktopInput()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:233:<tr class="memitem:ga00a5bf688d7556e8fe5bfec1b9608a17"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga00a5bf688d7556e8fe5bfec1b9608a17">TT_SendDesktopInput</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN const <a class="el" href="struct_desktop_input.html">DesktopInput</a> lpDesktopInputs[<a class="el" href="group__desktopshare.html#gaf4867025ab00e3b4852fde64971cb2ed">TT_DESKTOPINPUT_MAX</a>], IN INT32 nDesktopInputCount)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:338:<p>The remote user who wants to transmit mouse or keyboard input to the user sharing a desktop window can use <a class="el" href="group__desktopshare.html#ga00a5bf688d7556e8fe5bfec1b9608a17" title="Send a mouse or keyboard event to a shared desktop window.">TT_SendDesktopInput()</a>. Remember that the user sharing the desktop window must have enabled the subscription <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443bac180cbf89645f35df10e43eb88012e13" title="Subscribing to STREAMTYPE_DESKTOPINPUT.">SUBSCRIBE_DESKTOPINPUT</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:469:<p>The maximum number <a class="el" href="struct_desktop_input.html" title="A struct containing a mouse or keyboard event.">DesktopInput</a> instances which can be sent by <a class="el" href="group__desktopshare.html#ga00a5bf688d7556e8fe5bfec1b9608a17" title="Send a mouse or keyboard event to a shared desktop window.">TT_SendDesktopInput()</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1430:<h2 class="memtitle"><span class="permalink"><a href="#ga00a5bf688d7556e8fe5bfec1b9608a17">&#9670;&nbsp;</a></span>TT_SendDesktopInput()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1436:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SendDesktopInput </td>
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2403:    TEAMTALKDLL_API TTBOOL TT_SendDesktopInput(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\desktop.rs:19:        unsafe { ffi::api().TT_SendDesktopInput(self.ptr.0, user_id.0, input, 1) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SendDesktopWindow`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1726:<li><a class="el" href="group__desktopshare.html#gac3bbe4c3ae5e32bf63ead4f31b623621" title="Transmit a desktop window (bitmap) to users in the same channel.">TT_SendDesktopWindow()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2079:<p>The major new feature in the 4.3 release is the ability to share desktop applications. When sharing a desktop application you send a bitmap, using <a class="el" href="group__desktopshare.html#gac3bbe4c3ae5e32bf63ead4f31b623621" title="Transmit a desktop window (bitmap) to users in the same channel.">TT_SendDesktopWindow()</a>, to the local client instance. The bitmap is then split into in small blocks and transmitted to the server using the UDP connection. Read more about this feature in the section <a class="el" href="group__desktopshare.html">Desktop Sharing</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2121:<li><a class="el" href="group__desktopshare.html#gac3bbe4c3ae5e32bf63ead4f31b623621" title="Transmit a desktop window (bitmap) to users in the same channel.">TT_SendDesktopWindow()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2122:<li><a class="el" href="group__desktopshare.html#ga983f0c81ef934443bf690c14723b937c" title="Transmit the specified window in a desktop session.">TT_SendDesktopWindowFromHWND()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2185:<li><a class="el" href="group__initclient.html#gga58d6e380015b4b1c92c0f09fd6bcfc1ca840b3551969ef77dd96175de84ecd0d3" title="If set the client instance current have an active desktop session, i.e. TT_SendDesktopWindow() has be...">CLIENT_DESKTOP_ACTIVE</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:580:<li>TT_SendDesktopWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:583:<li>TT_SendDesktopWindowFromHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:649:<li>TT_SendDesktopWindow()
... (43 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2290:    TEAMTALKDLL_API INT32 TT_SendDesktopWindow(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2334:    TEAMTALKDLL_API INT32 TT_SendDesktopWindowFromHWND(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SendDesktopWindowFromHWND`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2122:<li><a class="el" href="group__desktopshare.html#ga983f0c81ef934443bf690c14723b937c" title="Transmit the specified window in a desktop session.">TT_SendDesktopWindowFromHWND()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:583:<li>TT_SendDesktopWindowFromHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:652:<li>TT_SendDesktopWindowFromHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:212:<tr class="memitem:ga983f0c81ef934443bf690c14723b937c"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga983f0c81ef934443bf690c14723b937c">TT_SendDesktopWindowFromHWND</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN HWND hWnd, IN <a class="el" href="group__desktopshare.html#ga23d1a7c7cf0f6da45ca389904e644d55">BitmapFormat</a> nBitmapFormat, IN <a class="el" href="group__desktopshare.html#ga774e2ca94287587e141c55736e38efe3">DesktopProtocol</a> nDesktopProtocol)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:301:<li><a class="el" href="group__desktopshare.html#ga983f0c81ef934443bf690c14723b937c" title="Transmit the specified window in a desktop session.">TT_SendDesktopWindowFromHWND()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1041:<h2 class="memtitle"><span class="permalink"><a href="#ga983f0c81ef934443bf690c14723b937c">&#9670;&nbsp;</a></span>TT_SendDesktopWindowFromHWND()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:1047:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> INT32 TT_SendDesktopWindowFromHWND </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:74:    [ "TT_SendDesktopWindowFromHWND", "group__desktopshare.html#ga983f0c81ef934443bf690c14723b937c", null ],
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2334:    TEAMTALKDLL_API INT32 TT_SendDesktopWindowFromHWND(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetClientKeepAlive`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:771:<p>Now it's possible to configure the keep alive properties using <a class="el" href="struct_client_keep_alive.html" title="Control timers for sending keep alive information to the server.">ClientKeepAlive</a>-struct and functions <a class="el" href="group__connectivity.html#gadf4cc840006b7c4f49caac2f63ad3e5f" title="Update the client instance&#39;s default keep alive settings.">TT_SetClientKeepAlive()</a> and <a class="el" href="group__connectivity.html#ga5dacbde76801d119b1045a87f4fa7c25" title="Get the client instance&#39;s current keep alive settings.">TT_GetClientKeepAlive()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:797:<li>New functions <a class="el" href="group__connectivity.html#gadf4cc840006b7c4f49caac2f63ad3e5f" title="Update the client instance&#39;s default keep alive settings.">TT_SetClientKeepAlive()</a> and <a class="el" href="group__connectivity.html#ga5dacbde76801d119b1045a87f4fa7c25" title="Get the client instance&#39;s current keep alive settings.">TT_GetClientKeepAlive()</a> for keep alive properties.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:586:<li>TT_SetClientKeepAlive()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:655:<li>TT_SetClientKeepAlive()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.js:56:    [ "TT_SetClientKeepAlive", "group__connectivity.html#gadf4cc840006b7c4f49caac2f63ad3e5f", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:150:<tr class="memitem:gadf4cc840006b7c4f49caac2f63ad3e5f"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__connectivity.html#gadf4cc840006b7c4f49caac2f63ad3e5f">TT_SetClientKeepAlive</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="struct_client_keep_alive.html">ClientKeepAlive</a> *lpClientKeepAlive)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:617:<h2 class="memtitle"><span class="permalink"><a href="#gadf4cc840006b7c4f49caac2f63ad3e5f">&#9670;&nbsp;</a></span>TT_SetClientKeepAlive()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:623:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SetClientKeepAlive </td>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2469:    TEAMTALKDLL_API TTBOOL TT_SetClientKeepAlive(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\connection.rs:581:        if unsafe { ffi::api().TT_SetClientKeepAlive(self.ptr.0, &keep_alive.to_ffi()) == 1 } {
```

### crates/teamtalk/tests
No matches

### docs
```text
D:\downloads\repos\TeamTalkRust\docs\changelog.md:65:- `Client::set_client_keep_alive()` now rejects invalid timeout relationships before calling `TT_SetClientKeepAlive`.
```

### README.md
No matches

## `TT_SetEncryptionContext`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:615:<p>Both client and server are now able to verify that the remote end is a valid user using TLS peer verification. Peer verfication is set up in <a class="el" href="struct_encryption_context.html" title="Configure peer verification for encrypted connection.">EncryptionContext</a> and enable using <a class="el" href="group__connectivity.html#gae5c3c59f5d71060f68e1266f25bd79e1" title="Setup encryption properties prior to TT_Connect().">TT_SetEncryptionContext()</a> on the client and <a class="el" href="group__serverapi.html#ga7d841aa79e2459e6c66a386d2c09ad80" title="Set certificate and private key for encrypted server.">TTS_SetEncryptionContext()</a> on the server.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:644:<li>New function <a class="el" href="group__connectivity.html#gae5c3c59f5d71060f68e1266f25bd79e1" title="Setup encryption properties prior to TT_Connect().">TT_SetEncryptionContext()</a> for setting up peer verification.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:589:<li>TT_SetEncryptionContext()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:658:<li>TT_SetEncryptionContext()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:129:<tr class="memitem:gae5c3c59f5d71060f68e1266f25bd79e1"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__connectivity.html#gae5c3c59f5d71060f68e1266f25bd79e1">TT_SetEncryptionContext</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, const <a class="el" href="struct_encryption_context.html">EncryptionContext</a> *lpEncryptionContext)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:181:<p>Call <a class="el" href="group__connectivity.html#gae5c3c59f5d71060f68e1266f25bd79e1" title="Setup encryption properties prior to TT_Connect().">TT_SetEncryptionContext()</a> to set up peer verification. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:238:<h2 class="memtitle"><span class="permalink"><a href="#gae5c3c59f5d71060f68e1266f25bd79e1">&#9670;&nbsp;</a></span>TT_SetEncryptionContext()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:244:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SetEncryptionContext </td>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2425:    TEAMTALKDLL_API TTBOOL TT_SetEncryptionContext(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\encryption.rs:22:        unsafe { ffi::api().TT_SetEncryptionContext(self.ptr.0, &context.to_ffi()) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetLicenseInformation`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:592:<li>TT_SetLicenseInformation()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:661:<li>TT_SetLicenseInformation()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:168:<tr class="memitem:ga601d31a2f571006bb6b4e97330c4c202"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__initclient.html#ga601d31a2f571006bb6b4e97330c4c202">TT_SetLicenseInformation</a> (IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szRegName[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>], IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> szRegKey[<a class="el" href="_team_talk_8h.html#a010c8742ded92e53cd997e33b788321b">TT_STRLEN</a>])</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:586:<h2 class="memtitle"><span class="permalink"><a href="#ga601d31a2f571006bb6b4e97330c4c202">&#9670;&nbsp;</a></span>TT_SetLicenseInformation()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:592:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SetLicenseInformation </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.js:36:    [ "TT_SetLicenseInformation", "group__initclient.html#ga601d31a2f571006bb6b4e97330c4c202", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\license.html:118:<p>To apply the TeamTalk 5 SDK license in a C API application call the function <a class="el" href="group__initclient.html#ga601d31a2f571006bb6b4e97330c4c202" title="Set license information to disable trial mode.">TT_SetLicenseInformation()</a>. This will disable trial mode.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\license.html:120:<div class="line">  <a class="code" href="group__initclient.html#ga601d31a2f571006bb6b4e97330c4c202">TT_SetLicenseInformation</a>(<span class="stringliteral">&quot;MyCompany&quot;</span>, <span class="stringliteral">&quot;1234abcd&quot;</span>);</div>
... (7 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1992:     TEAMTALKDLL_API TTBOOL TT_SetLicenseInformation(IN const TTCHAR szRegName[TT_STRLEN],
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\lib.rs:64:        teamtalk_sys::api().TT_SetLicenseInformation(name.tt().as_ptr(), key.tt().as_ptr()) == 1
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\system.rs:22:        unsafe { ffi::api().TT_SetLicenseInformation(name.tt().as_ptr(), key.tt().as_ptr()) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
```text
D:\downloads\repos\TeamTalkRust\docs\configuration.md:46:This matches TeamTalk C-API requirements (`TT_SetLicenseInformation` before
```

### README.md
No matches

## `TT_SetSoundDeviceEffects`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:662:<li>Enable this feature using <a class="el" href="group__sounddevices.html#ga1dcf7e8e7cfbcff184a920bb3c9e7609" title="Set up audio effects on a sound device.">TT_SetSoundDeviceEffects()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:667:<li>Enable this feature using <a class="el" href="group__sounddevices.html#ga1dcf7e8e7cfbcff184a920bb3c9e7609" title="Set up audio effects on a sound device.">TT_SetSoundDeviceEffects()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:672:<li>Enable this feature using <a class="el" href="group__sounddevices.html#ga1dcf7e8e7cfbcff184a920bb3c9e7609" title="Set up audio effects on a sound device.">TT_SetSoundDeviceEffects()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:685:<p><a class="el" href="group__sounddevices.html#ga1dcf7e8e7cfbcff184a920bb3c9e7609" title="Set up audio effects on a sound device.">TT_SetSoundDeviceEffects()</a> is a new function in TeamTalk v5.6 which can be used for enabling AGC, AEC and denoising on Windows and Android.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:718:<li>New function <a class="el" href="group__sounddevices.html#ga1dcf7e8e7cfbcff184a920bb3c9e7609" title="Set up audio effects on a sound device.">TT_SetSoundDeviceEffects()</a> for enabling <a class="el" href="struct_sound_device_effects.html" title="Set up audio effects supported by the sound device.">SoundDeviceEffects</a> on a client instance prior to initialization of the <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:595:<li>TT_SetSoundDeviceEffects()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:664:<li>TT_SetSoundDeviceEffects()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__codecs.html:673:<p><a class="el" href="struct_web_r_t_c_audio_preprocessor.html" title="WebRTC&#39;s audio preprocessor.">WebRTCAudioPreprocessor</a> is recommended to <a class="el" href="group__sounddevices.html#ga1dcf7e8e7cfbcff184a920bb3c9e7609" title="Set up audio effects on a sound device.">TT_SetSoundDeviceEffects()</a> on desktop platforms.</p>
... (24 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2066:    TEAMTALKDLL_API TTBOOL TT_SetSoundDeviceEffects(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:245:        unsafe { ffi::api().TT_SetSoundDeviceEffects(self.ptr.0, effects) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetSoundInputGainLevel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\annotated.html:121:<tr id="row_30_" class="even"><td class="entry"><span style="width:16px;display:inline-block;">&#160;</span><span class="icona"><span class="icon">C</span></span><a class="el" href="struct_t_t_audio_preprocessor.html" target="_self">TTAudioPreprocessor</a></td><td class="desc">Use TeamTalk's internal audio preprocessor for gain audio. Same as used for <a class="el" href="group__sounddevices.html#ga150161b6396bf215d6c8d637c47ccd05" title="Set voice gaining of recorded audio.">TT_SetSoundInputGainLevel()</a> </td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:752:<p>A new audio <a class="el" href="struct_t_t_audio_preprocessor.html" title="Use TeamTalk&#39;s internal audio preprocessor for gain audio. Same as used for TT_SetSoundInputGainLevel...">TTAudioPreprocessor</a> is introduced with can mute left/right audio channel and change gain level.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:837:<li>New <a class="el" href="struct_t_t_audio_preprocessor.html" title="Use TeamTalk&#39;s internal audio preprocessor for gain audio. Same as used for TT_SetSoundInputGainLevel...">TTAudioPreprocessor</a> struct for <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> or <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a>.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2514:<li>Call <a class="el" href="group__sounddevices.html#ga150161b6396bf215d6c8d637c47ccd05" title="Set voice gaining of recorded audio.">TT_SetSoundInputGainLevel</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:104:<dt>Member <a class="el" href="group__sounddevices.html#ga150161b6396bf215d6c8d637c47ccd05">TT_SetSoundInputGainLevel</a>  (IN TTInstance *lpTTInstance, IN INT32 nLevel)</dt>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:598:<li>TT_SetSoundInputGainLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:667:<li>TT_SetSoundInputGainLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__codecs.html:113:<tr class="memdesc:"><td class="mdescLeft">&#160;</td><td class="mdescRight">Use TeamTalk's internal audio preprocessor for gain audio. Same as used for <a class="el" href="group__sounddevices.html#ga150161b6396bf215d6c8d637c47ccd05" title="Set voice gaining of recorded audio.">TT_SetSoundInputGainLevel()</a>.  <a href="struct_t_t_audio_preprocessor.html#details">More...</a><br /></td></tr>
... (26 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2077:    TEAMTALKDLL_API TTBOOL TT_SetSoundInputGainLevel(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:140:        unsafe { ffi::api().TT_SetSoundInputGainLevel(self.ptr.0, level) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetSoundInputPreprocess`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:105:<dd><a class="anchor" id="_deprecated000005"></a>Use <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a> and <a class="el" href="group__codecs.html#gga4e1a1ab9b03812a7e4620a0f24dcca0da7584d1517c1679802bd5d70381c1e646" title="Use TeamTalk&#39;s internal audio preprocessor TTAudioPreprocessor.">TEAMTALK_AUDIOPREPROCESSOR</a>. </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:106:<dt>Member <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175">TT_SetSoundInputPreprocess</a>  (IN TTInstance *lpTTInstance, IN const <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a> *lpSpeexDSP)</dt>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:107:<dd><a class="anchor" id="_deprecated000007"></a>Use <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:695:<p>Previously only <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a> was available as <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> but now others can be chosing using <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:720:<li>New function <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a> for selecting an <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> instead of only <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a> in <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocess()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1016:<p>The <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a>-struct is used by <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocess()</a> and <a class="el" href="group__sounddevices.html#gaf5ccdd9356ea11cbe7a26655cc4cc5ef" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTest()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1149:<p>If a <a class="el" href="struct_channel.html" title="A struct containing the properties of a channel.">Channel</a> was configured with an <a class="el" href="struct_audio_config.html" title="Audio configuration for clients in a channel.">AudioConfig</a> in TeamTalk 4 then the client instance would automatically enable this audio configuration. This is no longer the case in TeamTalk 5. Now the client application must invoke <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocess()</a> manually.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1169:<li>See <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocess()</a>.</li>
... (43 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2084:    TEAMTALKDLL_API TTBOOL TT_SetSoundInputPreprocess(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2092:    TEAMTALKDLL_API TTBOOL TT_SetSoundInputPreprocessEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:230:        unsafe { ffi::api().TT_SetSoundInputPreprocessEx(self.ptr.0, &preprocessor.to_ffi()) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetSoundInputPreprocessEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:105:<dd><a class="anchor" id="_deprecated000005"></a>Use <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a> and <a class="el" href="group__codecs.html#gga4e1a1ab9b03812a7e4620a0f24dcca0da7584d1517c1679802bd5d70381c1e646" title="Use TeamTalk&#39;s internal audio preprocessor TTAudioPreprocessor.">TEAMTALK_AUDIOPREPROCESSOR</a>. </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:107:<dd><a class="anchor" id="_deprecated000007"></a>Use <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:695:<p>Previously only <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a> was available as <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> but now others can be chosing using <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:720:<li>New function <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a> for selecting an <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> instead of only <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a> in <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocess()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:604:<li>TT_SetSoundInputPreprocessEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:673:<li>TT_SetSoundInputPreprocessEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__codecs.html:674:<p>Activate <a class="el" href="struct_web_r_t_c_audio_preprocessor.html" title="WebRTC&#39;s audio preprocessor.">WebRTCAudioPreprocessor</a> by calling <a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__codecs.html:782:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga606c4e5f074d196e010aba1d60c937de" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocessEx()</a> </dd>
... (16 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2092:    TEAMTALKDLL_API TTBOOL TT_SetSoundInputPreprocessEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:230:        unsafe { ffi::api().TT_SetSoundInputPreprocessEx(self.ptr.0, &preprocessor.to_ffi()) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetSoundOutputMute`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2502:<li>Call <a class="el" href="group__sounddevices.html#gad83a50e6871a13f927cfee08c3e5cdca" title="Set all users mute.">TT_SetSoundOutputMute</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:607:<li>TT_SetSoundOutputMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:676:<li>TT_SetSoundOutputMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:618:<p>The sound system properties of the <code>lpTTInstance</code> will be used for playback, i.e. <a class="el" href="group__sounddevices.html#gad83a50e6871a13f927cfee08c3e5cdca" title="Set all users mute.">TT_SetSoundOutputMute()</a>, <a class="el" href="group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6" title="Set master volume.">TT_SetSoundOutputVolume()</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:269:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#gad83a50e6871a13f927cfee08c3e5cdca" title="Set all users mute.">TT_SetSoundOutputMute</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:107:    [ "TT_SetSoundOutputMute", "group__sounddevices.html#gad83a50e6871a13f927cfee08c3e5cdca", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:300:<tr class="memitem:gad83a50e6871a13f927cfee08c3e5cdca"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gad83a50e6871a13f927cfee08c3e5cdca">TT_SetSoundOutputMute</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bMuteAll)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:1998:<h2 class="memtitle"><span class="permalink"><a href="#gad83a50e6871a13f927cfee08c3e5cdca">&#9670;&nbsp;</a></span>TT_SetSoundOutputMute()</h2>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2107:    TEAMTALKDLL_API TTBOOL TT_SetSoundOutputMute(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:160:        unsafe { ffi::api().TT_SetSoundOutputMute(self.ptr.0, if mute { 1 } else { 0 }) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetSoundOutputVolume`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2494:<li>Call <a class="el" href="group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6" title="Set master volume.">TT_SetSoundOutputVolume</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:610:<li>TT_SetSoundOutputVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:679:<li>TT_SetSoundOutputVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:618:<p>The sound system properties of the <code>lpTTInstance</code> will be used for playback, i.e. <a class="el" href="group__sounddevices.html#gad83a50e6871a13f927cfee08c3e5cdca" title="Set all users mute.">TT_SetSoundOutputMute()</a>, <a class="el" href="group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6" title="Set master volume.">TT_SetSoundOutputVolume()</a> and <a class="el" href="group__sounddevices.html#ga7346ae42a09c6548b2d93dbaed030ae0" title="Initialize the sound output device (for audio playback).">TT_InitSoundOutputDevice()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:105:    [ "TT_SetSoundOutputVolume", "group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:294:<tr class="memitem:gae27a7449c6c9c0574af062f78e5285b6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6">TT_SetSoundOutputVolume</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nVolume)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:862:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6" title="Set master volume.">TT_SetSoundOutputVolume</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:871:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#gae27a7449c6c9c0574af062f78e5285b6" title="Set master volume.">TT_SetSoundOutputVolume</a> </dd>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2100:    TEAMTALKDLL_API TTBOOL TT_SetSoundOutputVolume(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:150:        unsafe { ffi::api().TT_SetSoundOutputVolume(self.ptr.0, volume) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserAudioStreamBufferSize`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:618:<p>Compensation for network jitter can now be enabled using <a class="el" href="struct_jitter_config.html" title="Configuration parameters for the Jitter Buffer.">JitterConfig</a> and activated using <a class="el" href="group__sounddevices.html#ga39ac356bbbe641609192a9082b972ecc" title="Set the configuration for de-jitter measures for a user.">TT_SetUserJitterControl()</a>. The jitter configuration allows a buffer to build up before playback starts. Remember to also increase the allowed buffer size by calling <a class="el" href="group__sounddevices.html#ga399d1985d184e8c845efa7ea806b768b" title="Change the amount of media data which can be buffered in the user&#39;s playback queue.">TT_SetUserAudioStreamBufferSize()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1698:<li><a class="el" href="group__sounddevices.html#ga399d1985d184e8c845efa7ea806b768b" title="Change the amount of media data which can be buffered in the user&#39;s playback queue.">TT_SetUserAudioStreamBufferSize()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1931:<li>Use <a class="el" href="group__sounddevices.html#ga399d1985d184e8c845efa7ea806b768b" title="Change the amount of media data which can be buffered in the user&#39;s playback queue.">TT_SetUserAudioStreamBufferSize()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:613:<li>TT_SetUserAudioStreamBufferSize()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:682:<li>TT_SetUserAudioStreamBufferSize()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:342:<tr class="memitem:ga399d1985d184e8c845efa7ea806b768b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga399d1985d184e8c845efa7ea806b768b">TT_SetUserAudioStreamBufferSize</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga6c16695e0994a2ee32d4e93c15daeaaa">StreamTypes</a> uStreamType, IN INT32 nMSec)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2469:<p>The result of jitter buffering is that playout frames will get buffered in the playout buffer. Make sure to also size the playout buffer for the expected jitter via <a class="el" href="group__sounddevices.html#ga399d1985d184e8c845efa7ea806b768b" title="Change the amount of media data which can be buffered in the user&#39;s playback queue.">TT_SetUserAudioStreamBufferSize</a></p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2796:<h2 class="memtitle"><span class="permalink"><a href="#ga399d1985d184e8c845efa7ea806b768b">&#9670;&nbsp;</a></span>TT_SetUserAudioStreamBufferSize()</h2>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2804:    TEAMTALKDLL_API TTBOOL TT_SetUserAudioStreamBufferSize(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:180:            ffi::api().TT_SetUserAudioStreamBufferSize(self.ptr.0, user_id.0, st, msec) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserJitterControl`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:618:<p>Compensation for network jitter can now be enabled using <a class="el" href="struct_jitter_config.html" title="Configuration parameters for the Jitter Buffer.">JitterConfig</a> and activated using <a class="el" href="group__sounddevices.html#ga39ac356bbbe641609192a9082b972ecc" title="Set the configuration for de-jitter measures for a user.">TT_SetUserJitterControl()</a>. The jitter configuration allows a buffer to build up before playback starts. Remember to also increase the allowed buffer size by calling <a class="el" href="group__sounddevices.html#ga399d1985d184e8c845efa7ea806b768b" title="Change the amount of media data which can be buffered in the user&#39;s playback queue.">TT_SetUserAudioStreamBufferSize()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:639:<li>New function <a class="el" href="group__sounddevices.html#ga39ac356bbbe641609192a9082b972ecc" title="Set the configuration for de-jitter measures for a user.">TT_SetUserJitterControl()</a> for enabling <a class="el" href="struct_jitter_config.html" title="Configuration parameters for the Jitter Buffer.">JitterConfig</a> on a <a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:616:<li>TT_SetUserJitterControl()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:685:<li>TT_SetUserJitterControl()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:232:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga39ac356bbbe641609192a9082b972ecc" title="Set the configuration for de-jitter measures for a user.">TT_SetUserJitterControl()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:735:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga39ac356bbbe641609192a9082b972ecc" title="Set the configuration for de-jitter measures for a user.">TT_SetUserJitterControl</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:115:    [ "TT_SetUserJitterControl", "group__sounddevices.html#ga39ac356bbbe641609192a9082b972ecc", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:324:<tr class="memitem:ga39ac356bbbe641609192a9082b972ecc"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga39ac356bbbe641609192a9082b972ecc">TT_SetUserJitterControl</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga8a65141d9ea4bf9d2e2377ed6b888a1d">StreamType</a> nStreamType, IN const <a class="el" href="struct_jitter_config.html">JitterConfig</a> *lpJitterConfig)</td></tr>
... (14 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2762:     TEAMTALKDLL_API TTBOOL TT_SetUserJitterControl(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:318:            ffi::api().TT_SetUserJitterControl(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserMediaStorageDir`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:621:<p><a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a> now support OPUS .ogg files on Windows. This allows playback of files recorded using <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a> or <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:641:<li>New variable <code>%starttick%</code> for audio file names. See <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:642:<li>New function <a class="el" href="group__sounddevices.html#gaf241140fd4a2fcdb0377959101ee2ff6" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDirEx()</a> for configuring when an audio log should be closed.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:886:<p>Record in MP3 format when using <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a> and <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1073:<p>Previously it has only been possible to record own audio streams by subscribing to them. Now it's, however, possible to call <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a> with user ID 0 to record own audio stream.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1863:<li>Use <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:688:<li>TT_SetUserMediaStorageDir()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:691:<li>TT_SetUserMediaStorageDirEx()
... (34 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2789:    TEAMTALKDLL_API TTBOOL TT_SetUserMediaStorageDir(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2796:    TEAMTALKDLL_API TTBOOL TT_SetUserMediaStorageDirEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\recording\user.rs:53:            ffi::api().TT_SetUserMediaStorageDir(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\recording\user.rs:73:            ffi::api().TT_SetUserMediaStorageDirEx(
```

### crates/teamtalk/tests
No matches

### docs
```text
D:\downloads\repos\TeamTalkRust\docs\recording\user.md:20:- Per-user recording is controlled by `TT_SetUserMediaStorageDir(Ex)` under the hood.
```

### README.md
No matches

## `TT_SetUserMediaStorageDirEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:642:<li>New function <a class="el" href="group__sounddevices.html#gaf241140fd4a2fcdb0377959101ee2ff6" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDirEx()</a> for configuring when an audio log should be closed.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:691:<li>TT_SetUserMediaStorageDirEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:622:<li>TT_SetUserMediaStorageDirEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:120:    [ "TT_SetUserMediaStorageDirEx", "group__sounddevices.html#gaf241140fd4a2fcdb0377959101ee2ff6", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:339:<tr class="memitem:gaf241140fd4a2fcdb0377959101ee2ff6"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gaf241140fd4a2fcdb0377959101ee2ff6">TT_SetUserMediaStorageDirEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szFolderPath, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szFileNameVars, IN <a class="el" href="group__mediastream.html#gad18559d169602e85d0ad68da6ef8593f">AudioFileFormat</a> uAFF, IN INT32 nStopRecordingExtraDelayMSec)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2726:<h2 class="memtitle"><span class="permalink"><a href="#gaf241140fd4a2fcdb0377959101ee2ff6">&#9670;&nbsp;</a></span>TT_SetUserMediaStorageDirEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2732:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SetUserMediaStorageDirEx </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:213:  ['tt_5fsetusermediastoragedirex_877',['TT_SetUserMediaStorageDirEx',['../group__sounddevices.html#gaf241140fd4a2fcdb0377959101ee2ff6',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2796:    TEAMTALKDLL_API TTBOOL TT_SetUserMediaStorageDirEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\recording\user.rs:73:            ffi::api().TT_SetUserMediaStorageDirEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserMute`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1766:<li><a class="el" href="group__sounddevices.html#ga1979525558288d81c9e3c7b565a94b0a" title="Mute a user.">TT_SetUserMute()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:625:<li>TT_SetUserMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:694:<li>TT_SetUserMute()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:113:    [ "TT_SetUserMute", "group__sounddevices.html#ga1979525558288d81c9e3c7b565a94b0a", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:318:<tr class="memitem:ga1979525558288d81c9e3c7b565a94b0a"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga1979525558288d81c9e3c7b565a94b0a">TT_SetUserMute</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga8a65141d9ea4bf9d2e2377ed6b888a1d">StreamType</a> nStreamType, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bMute)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2323:<h2 class="memtitle"><span class="permalink"><a href="#ga1979525558288d81c9e3c7b565a94b0a">&#9670;&nbsp;</a></span>TT_SetUserMute()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2329:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SetUserMute </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__users.html:253:<p>If on the other hand the user application wants to mute a user <a class="el" href="group__sounddevices.html#ga1979525558288d81c9e3c7b565a94b0a" title="Mute a user.">TT_SetUserMute</a> can be used for this. Note that muting a user doesn't mean that the client instance will stop receiving audio from that user, it simply means it will not be played. To stop receiving audio from a user the local client instance must ask the server to unsubscribe voice data from the user. This is explained in the next section.</p>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2750:    TEAMTALKDLL_API TTBOOL TT_SetUserMute(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:166:            ffi::api().TT_SetUserMute(self.ptr.0, user_id.0, stream_type, if mute { 1 } else { 0 })
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserPosition`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:677:<li>Enable this feature using <a class="el" href="group__sounddevices.html#ga8c8370192f89d0d3c811b41a6499da16" title="Set the position of a user.">TT_SetUserPosition()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:628:<li>TT_SetUserPosition()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:697:<li>TT_SetUserPosition()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:272:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga8c8370192f89d0d3c811b41a6499da16" title="Set the position of a user.">TT_SetUserPosition()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:330:<tr class="memitem:ga8c8370192f89d0d3c811b41a6499da16"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga8c8370192f89d0d3c811b41a6499da16">TT_SetUserPosition</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga8a65141d9ea4bf9d2e2377ed6b888a1d">StreamType</a> nStreamType, IN float x, IN float y, IN float z)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:819:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga8c8370192f89d0d3c811b41a6499da16" title="Set the position of a user.">TT_SetUserPosition()</a> <br  />
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2072:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga8c8370192f89d0d3c811b41a6499da16" title="Set the position of a user.">TT_SetUserPosition()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2101:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__sounddevices.html#ga8c8370192f89d0d3c811b41a6499da16" title="Set the position of a user.">TT_SetUserPosition()</a> </dd></dl>
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2774:    TEAMTALKDLL_API TTBOOL TT_SetUserPosition(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:279:        unsafe { ffi::api().TT_SetUserPosition(self.ptr.0, user_id.0, stream_type, x, y, z) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserStereo`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:700:<li>TT_SetUserStereo()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:631:<li>TT_SetUserStereo()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:118:    [ "TT_SetUserStereo", "group__sounddevices.html#gad82b5ebaf4eb9d0aae4c7a40039dfd0a", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:333:<tr class="memitem:gad82b5ebaf4eb9d0aae4c7a40039dfd0a"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gad82b5ebaf4eb9d0aae4c7a40039dfd0a">TT_SetUserStereo</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga8a65141d9ea4bf9d2e2377ed6b888a1d">StreamType</a> nStreamType, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bLeftSpeaker, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bRightSpeaker)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2602:<h2 class="memtitle"><span class="permalink"><a href="#gad82b5ebaf4eb9d0aae4c7a40039dfd0a">&#9670;&nbsp;</a></span>TT_SetUserStereo()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2608:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SetUserStereo </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:216:  ['tt_5fsetuserstereo_880',['TT_SetUserStereo',['../group__sounddevices.html#gad82b5ebaf4eb9d0aae4c7a40039dfd0a',1,'TeamTalk.h']]],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\struct_speex_codec.html:191:<a class="el" href="group__sounddevices.html#gad82b5ebaf4eb9d0aae4c7a40039dfd0a" title="Set whether a user should speak in the left, right or both speakers. This function only works if Audi...">TT_SetUserStereo()</a> </dd></dl>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2782:    TEAMTALKDLL_API TTBOOL TT_SetUserStereo(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:291:            ffi::api().TT_SetUserStereo(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserStoppedPlaybackDelay`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1694:<li><a class="el" href="group__sounddevices.html#gae7b3588ecab306dc0eef34ede75d1d7b" title="Set the delay of when a user should no longer be considered as playing audio (either voice or audio f...">TT_SetUserStoppedPlaybackDelay()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1915:<li>Use <a class="el" href="group__sounddevices.html#gae7b3588ecab306dc0eef34ede75d1d7b" title="Set the delay of when a user should no longer be considered as playing audio (either voice or audio f...">TT_SetUserStoppedPlaybackDelay()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:634:<li>TT_SetUserStoppedPlaybackDelay()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:703:<li>TT_SetUserStoppedPlaybackDelay()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:114:    [ "TT_SetUserStoppedPlaybackDelay", "group__sounddevices.html#gae7b3588ecab306dc0eef34ede75d1d7b", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:321:<tr class="memitem:gae7b3588ecab306dc0eef34ede75d1d7b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gae7b3588ecab306dc0eef34ede75d1d7b">TT_SetUserStoppedPlaybackDelay</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga8a65141d9ea4bf9d2e2377ed6b888a1d">StreamType</a> nStreamType, IN INT32 nDelayMSec)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2376:<h2 class="memtitle"><span class="permalink"><a href="#gae7b3588ecab306dc0eef34ede75d1d7b">&#9670;&nbsp;</a></span>TT_SetUserStoppedPlaybackDelay()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:2382:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SetUserStoppedPlaybackDelay </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2756:    TEAMTALKDLL_API TTBOOL TT_SetUserStoppedPlaybackDelay(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:192:            ffi::api().TT_SetUserStoppedPlaybackDelay(self.ptr.0, user_id.0, stream_type, msec) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetUserVolume`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1762:<li><a class="el" href="group__sounddevices.html#gab1826616267c007816091ec4f24d0838" title="Set the volume of a user.">TT_SetUserVolume()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1795:<li>Use <a class="el" href="group__sounddevices.html#gab1826616267c007816091ec4f24d0838" title="Set the volume of a user.">TT_SetUserVolume()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:637:<li>TT_SetUserVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:706:<li>TT_SetUserVolume()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:208:<p>To stream a media file to a channel call <a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a> and to stop the stream call <a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2" title="Stop streaming media file to channel.">TT_StopStreamingMediaFileToChannel()</a>. The user receiving the media stream can control volume levels by calling <a class="el" href="group__sounddevices.html#gab1826616267c007816091ec4f24d0838" title="Set the volume of a user.">TT_SetUserVolume()</a> and <a class="el" href="group__mediastream.html#gab236763cba33f650ded61d2efe880fe3" title="Extract a user&#39;s media video frame for display.">TT_AcquireUserMediaVideoFrame()</a> to obtain video frames.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.js:112:    [ "TT_SetUserVolume", "group__sounddevices.html#gab1826616267c007816091ec4f24d0838", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:315:<tr class="memitem:gab1826616267c007816091ec4f24d0838"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#gab1826616267c007816091ec4f24d0838">TT_SetUserVolume</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nUserID, IN <a class="el" href="group__transmission.html#ga8a65141d9ea4bf9d2e2377ed6b888a1d">StreamType</a> nStreamType, IN INT32 nVolume)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:866:<a class="el" href="group__sounddevices.html#gab1826616267c007816091ec4f24d0838" title="Set the volume of a user.">TT_SetUserVolume</a> </dd>
... (15 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2744:    TEAMTALKDLL_API TTBOOL TT_SetUserVolume(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:458:        unsafe { ffi::api().TT_SetUserVolume(self.ptr.0, user_id.0, stream_type, volume) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetVoiceActivationLevel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:709:<li>TT_SetVoiceActivationLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:640:<li>TT_SetVoiceActivationLevel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__events.html:659:<a class="el" href="group__transmission.html#ga6936fcb85fbb4ea7feb9f10ccd581147" title="Set voice activation level.">TT_SetVoiceActivationLevel()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:259:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__transmission.html#ga6936fcb85fbb4ea7feb9f10ccd581147" title="Set voice activation level.">TT_SetVoiceActivationLevel</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:264:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__transmission.html#ga6936fcb85fbb4ea7feb9f10ccd581147" title="Set voice activation level.">TT_SetVoiceActivationLevel()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:21:    [ "TT_SetVoiceActivationLevel", "group__transmission.html#ga6936fcb85fbb4ea7feb9f10ccd581147", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:850:<a class="el" href="group__transmission.html#ga6936fcb85fbb4ea7feb9f10ccd581147" title="Set voice activation level.">TT_SetVoiceActivationLevel</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:857:<a class="el" href="group__transmission.html#ga6936fcb85fbb4ea7feb9f10ccd581147" title="Set voice activation level.">TT_SetVoiceActivationLevel</a> </dd>
... (14 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2147:    TEAMTALKDLL_API TTBOOL TT_SetVoiceActivationLevel(IN TTInstance* lpTTInstance, 
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:210:        unsafe { ffi::api().TT_SetVoiceActivationLevel(self.ptr.0, level) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SetVoiceActivationStopDelay`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2226:<p>Voice activation is by default disabled if no audio has reached the voice activation level for 1.5 seconds. This option can now be changed by calling <a class="el" href="group__transmission.html#gab401567bcf926c34e1b80d50fc9b0811" title="Set the delay of when voice activation should be stopped.">TT_SetVoiceActivationStopDelay()</a>. Also users who are talking are set to non-talking after 0.5 seconds if no new voice data has been received. This value can now be changed by calling <code>TT_SetUserStoppedTalkingDelay()</code>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2240:<li><a class="el" href="group__transmission.html#gab401567bcf926c34e1b80d50fc9b0811" title="Set the delay of when voice activation should be stopped.">TT_SetVoiceActivationStopDelay()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:643:<li>TT_SetVoiceActivationStopDelay()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:712:<li>TT_SetVoiceActivationStopDelay()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:23:    [ "TT_SetVoiceActivationStopDelay", "group__transmission.html#gab401567bcf926c34e1b80d50fc9b0811", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:144:<tr class="memitem:gab401567bcf926c34e1b80d50fc9b0811"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#gab401567bcf926c34e1b80d50fc9b0811">TT_SetVoiceActivationStopDelay</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nDelayMSec)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:401:<a class="el" href="group__transmission.html#gab401567bcf926c34e1b80d50fc9b0811" title="Set the delay of when voice activation should be stopped.">TT_SetVoiceActivationStopDelay</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:444:<a class="el" href="group__transmission.html#gab401567bcf926c34e1b80d50fc9b0811" title="Set the delay of when voice activation should be stopped.">TT_SetVoiceActivationStopDelay</a> </dd></dl>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2154:    TEAMTALKDLL_API TTBOOL TT_SetVoiceActivationStopDelay(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:220:        unsafe { ffi::api().TT_SetVoiceActivationStopDelay(self.ptr.0, delay) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartRecordingMuxedAudioFile`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:621:<p><a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a> now support OPUS .ogg files on Windows. This allows playback of files recorded using <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a> or <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:763:<p>Previously <a class="el" href="group__sounddevices.html#ga332b045b503ea31646fd26072e0e6da2" title="Enable/disable access to raw audio from individual users, local microphone input or mixed stream of a...">TT_EnableAudioBlockEvent()</a> could only be used to access audio from a single user. Using <a class="el" href="group__sounddevices.html#gaae84cd30592b71d2b43c37b7e414ca2e" title="User ID used to identify muxed audio that has been mixed into a single stream.">TT_MUXED_USERID</a> now makes it possible to access the audio stream where all users' audio streams have been mixed together. Basically the same as recording all conversations to a single file using <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:774:<p>Previously it was only possible to have one active audio recording using <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:775:<p>Now it's possible to record multiple channels at the same time using <a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113" title="Store audio conversations from a specific channel into a single file.">TT_StartRecordingMuxedAudioFileEx()</a>. Note that in order to get audio from channels where the TeamTalk instance is currently not participating requires the use of <a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d" title="Subscribe to user events and/or data.">TT_DoSubscribe()</a> and <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443ba304cea831425da3b9c0816dc96ae5015" title="Intercept all voice sent by a user. Only user-type USERTYPE_ADMIN can do this. By enabling this subsc...">SUBSCRIBE_INTERCEPT_VOICE</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:865:<li>New function <a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113" title="Store audio conversations from a specific channel into a single file.">TT_StartRecordingMuxedAudioFileEx()</a> for recording audio from channel.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:886:<p>Record in MP3 format when using <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a> and <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:955:<p>In order to record conversations using <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a> it has previously been required to initialize a real sound output device in order to process audio packets. It is still required to initialize the sound output device but now a new virtual sound device is available which processes audio packets. This virtual sound device has ID <a class="el" href="group__sounddevices.html#ga43d3d24a9c64a7cfcf52094c024e4dcf" title="Sound device ID for virtual TeamTalk sound device.">TT_SOUNDDEVICE_ID_TEAMTALK_VIRTUAL</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2220:<p>It is now possible to store "muxed" audio files, i.e. audio from all users are written to the same audio file. Check out <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a> on how to do this.</p>
... (43 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2161:    TEAMTALKDLL_API TTBOOL TT_StartRecordingMuxedAudioFile(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2167:    TEAMTALKDLL_API TTBOOL TT_StartRecordingMuxedAudioFileEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:197:            ffi::api().TT_StartRecordingMuxedAudioFile(ptr, &raw_codec, p.as_ptr(), format) == 1
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:210:            ffi::api().TT_StartRecordingMuxedAudioFileEx(ptr, channel_id, p.as_ptr(), format) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartRecordingMuxedAudioFileEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:775:<p>Now it's possible to record multiple channels at the same time using <a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113" title="Store audio conversations from a specific channel into a single file.">TT_StartRecordingMuxedAudioFileEx()</a>. Note that in order to get audio from channels where the TeamTalk instance is currently not participating requires the use of <a class="el" href="group__commands.html#ga54fb7c84fa6707f11f385709456ae94d" title="Subscribe to user events and/or data.">TT_DoSubscribe()</a> and <a class="el" href="group__users.html#ggaab1ec4ba26a015b2d65e3b900be8443ba304cea831425da3b9c0816dc96ae5015" title="Intercept all voice sent by a user. Only user-type USERTYPE_ADMIN can do this. By enabling this subsc...">SUBSCRIBE_INTERCEPT_VOICE</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:865:<li>New function <a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113" title="Store audio conversations from a specific channel into a single file.">TT_StartRecordingMuxedAudioFileEx()</a> for recording audio from channel.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:649:<li>TT_StartRecordingMuxedAudioFileEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:739:<li>TT_StartRecordingMuxedAudioFileEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:153:<tr class="memitem:gaa941f3e819cbf98c05639ec03d75c113"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113">TT_StartRecordingMuxedAudioFileEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szAudioFileName, IN <a class="el" href="group__mediastream.html#gad18559d169602e85d0ad68da6ef8593f">AudioFileFormat</a> uAFF)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:577:<p>To record conversations from a specific channel to a single file call <a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113" title="Store audio conversations from a specific channel into a single file.">TT_StartRecordingMuxedAudioFileEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:596:<a class="el" href="group__transmission.html#gaa941f3e819cbf98c05639ec03d75c113" title="Store audio conversations from a specific channel into a single file.">TT_StartRecordingMuxedAudioFileEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:605:<h2 class="memtitle"><span class="permalink"><a href="#gaa941f3e819cbf98c05639ec03d75c113">&#9670;&nbsp;</a></span>TT_StartRecordingMuxedAudioFileEx()</h2>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2167:    TEAMTALKDLL_API TTBOOL TT_StartRecordingMuxedAudioFileEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:210:            ffi::api().TT_StartRecordingMuxedAudioFileEx(ptr, channel_id, p.as_ptr(), format) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartRecordingMuxedStreams`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:588:<p>To record multiple audio streams, i.e. <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dae5064c6cd0444d6e4f46598eaf4fb018" title="Voice stream type which is audio recorded from a sound input device.">STREAMTYPE_VOICE</a>, <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dabbaecd785019d0eadc798e99d753b32b" title="Stream type for audio of local playback.">STREAMTYPE_LOCALMEDIAPLAYBACK_AUDIO</a> and <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1da143d62c7209b63caf2d9a10f67bbb1fb" title="Audio stream type from a media file which is being streamed.">STREAMTYPE_MEDIAFILE_AUDIO</a>, use <a class="el" href="group__transmission.html#ga72ffa52d97f624812144ce06072fd7f0" title="Mix multiple StreamTypes into a single audio file.">TT_StartRecordingMuxedStreams()</a>. The <a class="el" href="group__transmission.html#ga6c16695e0994a2ee32d4e93c15daeaaa" title="Mask of StreamType.">StreamTypes</a> can be OR'ed to mix the wanted combination.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:595:<li>New function <a class="el" href="group__transmission.html#ga72ffa52d97f624812144ce06072fd7f0" title="Mix multiple StreamTypes into a single audio file.">TT_StartRecordingMuxedStreams()</a> to record multiple <a class="el" href="group__transmission.html#ga6c16695e0994a2ee32d4e93c15daeaaa" title="Mask of StreamType.">StreamTypes</a> to a single file.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:652:<li>TT_StartRecordingMuxedStreams()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:742:<li>TT_StartRecordingMuxedStreams()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:27:    [ "TT_StartRecordingMuxedStreams", "group__transmission.html#ga72ffa52d97f624812144ce06072fd7f0", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:156:<tr class="memitem:ga72ffa52d97f624812144ce06072fd7f0"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#ga72ffa52d97f624812144ce06072fd7f0">TT_StartRecordingMuxedStreams</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN <a class="el" href="group__transmission.html#ga6c16695e0994a2ee32d4e93c15daeaaa">StreamTypes</a> uStreamTypes, IN const <a class="el" href="struct_audio_codec.html">AudioCodec</a> *lpAudioCodec, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szAudioFileName, IN <a class="el" href="group__mediastream.html#gad18559d169602e85d0ad68da6ef8593f">AudioFileFormat</a> uAFF)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:582:<p>Only <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1dae5064c6cd0444d6e4f46598eaf4fb018" title="Voice stream type which is audio recorded from a sound input device.">STREAMTYPE_VOICE</a> is stored into the audio file, not <a class="el" href="group__transmission.html#gga8a65141d9ea4bf9d2e2377ed6b888a1da143d62c7209b63caf2d9a10f67bbb1fb" title="Audio stream type from a media file which is being streamed.">STREAMTYPE_MEDIAFILE_AUDIO</a>. To record other stream types use <a class="el" href="group__transmission.html#ga72ffa52d97f624812144ce06072fd7f0" title="Mix multiple StreamTypes into a single audio file.">TT_StartRecordingMuxedStreams()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:583:<p><a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a> is mutually exclusive with <a class="el" href="group__transmission.html#ga72ffa52d97f624812144ce06072fd7f0" title="Mix multiple StreamTypes into a single audio file.">TT_StartRecordingMuxedStreams()</a>.</p>
... (11 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2173:    TEAMTALKDLL_API TTBOOL TT_StartRecordingMuxedStreams(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:225:            ffi::api().TT_StartRecordingMuxedStreams(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartSoundLoopbackTest`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:692:<p>When <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> was introduced it could not be used when performing a loopback test. This is now possible using <a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:715:<li>New function <a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a> for testing a <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a> with <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> and <a class="el" href="struct_sound_device_effects.html" title="Set up audio effects supported by the sound device.">SoundDeviceEffects</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1016:<p>The <a class="el" href="struct_speex_d_s_p.html" title="Speex DSP is used for specifying how recorded audio from a sound input device should be preprocessed ...">SpeexDSP</a>-struct is used by <a class="el" href="group__sounddevices.html#gae62d2856d608c9adebf5b586159fb175" title="Enable sound preprocessor which should be used for processing audio recorded by the sound input devic...">TT_SetSoundInputPreprocess()</a> and <a class="el" href="group__sounddevices.html#gaf5ccdd9356ea11cbe7a26655cc4cc5ef" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTest()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1714:<li><a class="el" href="group__sounddevices.html#gaf5ccdd9356ea11cbe7a26655cc4cc5ef" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTest()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1715:<li>Now returns <a class="el" href="group__sounddevices.html#ga0b90a2b9785ff1fc52667e5673de800e" title="Pointer to a sound loop for testing sound devices created by TT_StartSoundLoopbackTest()">TTSoundLoop</a> instance.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1790:<li><code><a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a></code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1791:<li>Use <a class="el" href="group__sounddevices.html#gaf5ccdd9356ea11cbe7a26655cc4cc5ef" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTest()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2317:<li><code><a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a></code> <ul>
... (35 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2015:    TEAMTALKDLL_API TTSoundLoop* TT_StartSoundLoopbackTest(IN INT32 nInputDeviceID,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2023:    TEAMTALKDLL_API TTSoundLoop* TT_StartSoundLoopbackTestEx(IN INT32 nInputDeviceID,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:392:            ffi::api().TT_StartSoundLoopbackTestEx(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:471:            ffi::api().TT_StartSoundLoopbackTest(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartSoundLoopbackTestEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\deprecated.html:109:<dd><a class="anchor" id="_deprecated000004"></a>Use <a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:692:<p>When <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> was introduced it could not be used when performing a loopback test. This is now possible using <a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:715:<li>New function <a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a> for testing a <a class="el" href="struct_sound_device.html" title="A struct containing the properties of a sound device for either playback or recording.">SoundDevice</a> with <a class="el" href="struct_audio_preprocessor.html" title="Configure the audio preprocessor specified by nPreprocessor.">AudioPreprocessor</a> and <a class="el" href="struct_sound_device_effects.html" title="Set up audio effects supported by the sound device.">SoundDeviceEffects</a>.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1790:<li><code><a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a></code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2317:<li><code><a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97" title="Perform a record and playback test of specified sound devices along with an audio configuration.">TT_StartSoundLoopbackTestEx()</a></code> <ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:748:<li>TT_StartSoundLoopbackTestEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:658:<li>TT_StartSoundLoopbackTestEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__sounddevices.html:237:<tr class="memitem:ga6fb9cc73bdfba33e55fda14466857d97"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="group__sounddevices.html#ga0b90a2b9785ff1fc52667e5673de800e">TTSoundLoop</a> *&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__sounddevices.html#ga6fb9cc73bdfba33e55fda14466857d97">TT_StartSoundLoopbackTestEx</a> (IN INT32 nInputDeviceID, IN INT32 nOutputDeviceID, IN INT32 nSampleRate, IN INT32 nChannels, IN <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> bDuplexMode, IN const <a class="el" href="struct_audio_preprocessor.html">AudioPreprocessor</a> *lpAudioPreprocessor, IN const <a class="el" href="struct_sound_device_effects.html">SoundDeviceEffects</a> *lpSoundDeviceEffects)</td></tr>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2023:    TEAMTALKDLL_API TTSoundLoop* TT_StartSoundLoopbackTestEx(IN INT32 nInputDeviceID,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\audio.rs:392:            ffi::api().TT_StartSoundLoopbackTestEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartStreamingMediaFileToChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:621:<p><a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a> now support OPUS .ogg files on Windows. This allows playback of files recorded using <a class="el" href="group__sounddevices.html#gadbf65ee87b3729a231c639befeb54dbc" title="Store user&#39;s audio to disk.">TT_SetUserMediaStorageDir()</a> or <a class="el" href="group__transmission.html#gaec428c3176a3504af5a55aaca7b1f741" title="Store all audio conversations with specific AudioCodec settings to a single file.">TT_StartRecordingMuxedAudioFile()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:754:<p>To use these new features use <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a> instead of <a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:837:<li>New <a class="el" href="struct_t_t_audio_preprocessor.html" title="Use TeamTalk&#39;s internal audio preprocessor for gain audio. Same as used for TT_SetSoundInputGainLevel...">TTAudioPreprocessor</a> struct for <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> or <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a>.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:841:<li>New function <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a> for streaming media file.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1599:<li><code>TT_StartStreamingAudioFileToUser()</code> replaced by <a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1855:<li>Use <a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1987:<p>Previously it's only been possible to stream 16-bit signed PCM wave-files to a channel but in the 4.5 release it's now possible to stream mp3, mpg, avi, wma, wmv, etc. to a channel. On Windows you can basically stream whatever Windows Media Player can play to a channel. Checkout <code><a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a></code> for more information.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2017:<li><a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a></li>
... (46 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2242:    TEAMTALKDLL_API TTBOOL TT_StartStreamingMediaFileToChannel(IN TTInstance* lpTTInstance,
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2247:    TEAMTALKDLL_API TTBOOL TT_StartStreamingMediaFileToChannelEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:33:            ffi::api().TT_StartStreamingMediaFileToChannel(
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:50:            ffi::api().TT_StartStreamingMediaFileToChannelEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartStreamingMediaFileToChannelEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:754:<p>To use these new features use <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a> instead of <a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:837:<li>New <a class="el" href="struct_t_t_audio_preprocessor.html" title="Use TeamTalk&#39;s internal audio preprocessor for gain audio. Same as used for TT_SetSoundInputGainLevel...">TTAudioPreprocessor</a> struct for <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> or <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a>.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:841:<li>New function <a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a> for streaming media file.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:664:<li>TT_StartStreamingMediaFileToChannelEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:754:<li>TT_StartStreamingMediaFileToChannelEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.js:54:    [ "TT_StartStreamingMediaFileToChannelEx", "group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:177:<tr class="memitem:ga3ab48ec14490f3893210ee47aeb4293a"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a">TT_StartStreamingMediaFileToChannelEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="_team_talk_8h.html#aa39e784ac0d6402ca86a9aa27db00764">TTCHAR</a> *szMediaFilePath, IN const <a class="el" href="struct_media_file_playback.html">MediaFilePlayback</a> *lpMediaFilePlayback, IN const <a class="el" href="struct_video_codec.html">VideoCodec</a> *lpVideoCodec)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:316:<a class="el" href="group__mediastream.html#ga3ab48ec14490f3893210ee47aeb4293a" title="Stream media file to channel, e.g. avi, wav or MP3-file.">TT_StartStreamingMediaFileToChannelEx()</a> </dd>
... (10 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2247:    TEAMTALKDLL_API TTBOOL TT_StartStreamingMediaFileToChannelEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:50:            ffi::api().TT_StartStreamingMediaFileToChannelEx(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StartVideoCaptureTransmission`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1358:<li><code>USERRIGHT_FORWARD_VIDEO</code> replaced by <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679ab57030f23de524c7fdf647c847c960fb" title="User is allowed to forward video packets through server. TT_StartVideoCaptureTransmission()">USERRIGHT_TRANSMIT_VIDEOCAPTURE</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1642:<li><a class="el" href="group__transmission.html#ga1e76ef6ae7f72331dff1dbd9880baaa4" title="Start transmitting from video capture device.">TT_StartVideoCaptureTransmission()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1839:<li>Use <a class="el" href="group__transmission.html#ga8ef1203cd2998908c95761c2621b573c" title="Start/stop transmitting of voice data from sound input.">TT_EnableVoiceTransmission()</a> or <a class="el" href="group__transmission.html#ga1e76ef6ae7f72331dff1dbd9880baaa4" title="Start transmitting from video capture device.">TT_StartVideoCaptureTransmission()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:757:<li>TT_StartVideoCaptureTransmission()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:667:<li>TT_StartVideoCaptureTransmission()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__connectivity.html:165:<p>If the server should not allow clients to forward audio and video packets the <em>uUserRights</em> member of <a class="el" href="struct_user_account.html" title="A struct containing the properties of a user account.">UserAccount</a> must disable <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679afc11323082ea6f7667a9f4368885b058" title="Users are allowed to forward audio packets through server. TT_EnableVoiceTransmission()">USERRIGHT_TRANSMIT_VOICE</a> and <a class="el" href="group__server.html#ggaa62615f8034ace22e5dd6dfa6778e679ab57030f23de524c7fdf647c847c960fb" title="User is allowed to forward video packets through server. TT_StartVideoCaptureTransmission()">USERRIGHT_TRANSMIT_VIDEOCAPTURE</a>. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__server.html:535:<tr><td class="fieldname"><a id="ggaa62615f8034ace22e5dd6dfa6778e679ab57030f23de524c7fdf647c847c960fb"></a>USERRIGHT_TRANSMIT_VIDEOCAPTURE&#160;</td><td class="fielddoc"><p><a class="el" href="struct_user.html" title="A struct containing the properties of a user.">User</a> is allowed to forward video packets through server. <a class="el" href="group__transmission.html#ga1e76ef6ae7f72331dff1dbd9880baaa4" title="Start transmitting from video capture device.">TT_StartVideoCaptureTransmission()</a> </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:282:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__transmission.html#ga1e76ef6ae7f72331dff1dbd9880baaa4" title="Start transmitting from video capture device.">TT_StartVideoCaptureTransmission()</a> </dd></dl>
... (16 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2187:    TEAMTALKDLL_API TTBOOL TT_StartVideoCaptureTransmission(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:58:        unsafe { ffi::api().TT_StartVideoCaptureTransmission(self.ptr.0, &codec.to_ffi()) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StopLocalPlayback`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:747:<p>Stop local media file playback using <a class="el" href="group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49">TT_StopLocalPlayback()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:817:<li>New functions <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a>, <a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback()</a> and <a class="el" href="group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49">TT_StopLocalPlayback()</a> for local media playback.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:760:<li>TT_StopLocalPlayback()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:670:<li>TT_StopLocalPlayback()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.js:59:    [ "TT_StopLocalPlayback", "group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:191:<tr class="memitem:ga65ca66b1ee8b9b907e489c3dfd3fda49"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49">TT_StopLocalPlayback</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nPlaybackSessionID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:633:<a class="el" href="group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49">TT_StopLocalPlayback()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:681:<a class="el" href="group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49">TT_StopLocalPlayback()</a> </dd>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2271:    TEAMTALKDLL_API TTBOOL TT_StopLocalPlayback(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:96:        unsafe { ffi::api().TT_StopLocalPlayback(self.ptr.0, session_id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StopRecordingMuxedAudioFile`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:869:<li>New function <a class="el" href="group__transmission.html#ga7c0a438711d04834fd9e9ea9d4bddd40" title="Stop recording conversations from a channel to a single file.">TT_StopRecordingMuxedAudioFileEx()</a> for stopping recording audio from channel.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2243:<li><a class="el" href="group__transmission.html#ga8cafb3c3867333c72806264107d9315a" title="Stop an active muxed audio recording.">TT_StopRecordingMuxedAudioFile()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:673:<li>TT_StopRecordingMuxedAudioFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:676:<li>TT_StopRecordingMuxedAudioFileEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:763:<li>TT_StopRecordingMuxedAudioFile()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:766:<li>TT_StopRecordingMuxedAudioFileEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:28:    [ "TT_StopRecordingMuxedAudioFile", "group__transmission.html#ga8cafb3c3867333c72806264107d9315a", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:29:    [ "TT_StopRecordingMuxedAudioFileEx", "group__transmission.html#ga7c0a438711d04834fd9e9ea9d4bddd40", null ],
... (25 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2180:    TEAMTALKDLL_API TTBOOL TT_StopRecordingMuxedAudioFile(IN TTInstance* lpTTInstance);
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2183:    TEAMTALKDLL_API TTBOOL TT_StopRecordingMuxedAudioFileEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:236:        unsafe { ffi::api().TT_StopRecordingMuxedAudioFile(ptr) == 1 }
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:240:        unsafe { ffi::api().TT_StopRecordingMuxedAudioFileEx(ptr, channel_id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StopRecordingMuxedAudioFileEx`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:869:<li>New function <a class="el" href="group__transmission.html#ga7c0a438711d04834fd9e9ea9d4bddd40" title="Stop recording conversations from a channel to a single file.">TT_StopRecordingMuxedAudioFileEx()</a> for stopping recording audio from channel.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:676:<li>TT_StopRecordingMuxedAudioFileEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:766:<li>TT_StopRecordingMuxedAudioFileEx()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:29:    [ "TT_StopRecordingMuxedAudioFileEx", "group__transmission.html#ga7c0a438711d04834fd9e9ea9d4bddd40", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:162:<tr class="memitem:ga7c0a438711d04834fd9e9ea9d4bddd40"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#ga7c0a438711d04834fd9e9ea9d4bddd40">TT_StopRecordingMuxedAudioFileEx</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nChannelID)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:660:<a class="el" href="group__transmission.html#ga7c0a438711d04834fd9e9ea9d4bddd40" title="Stop recording conversations from a channel to a single file.">TT_StopRecordingMuxedAudioFileEx()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:770:<h2 class="memtitle"><span class="permalink"><a href="#ga7c0a438711d04834fd9e9ea9d4bddd40">&#9670;&nbsp;</a></span>TT_StopRecordingMuxedAudioFileEx()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:776:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_StopRecordingMuxedAudioFileEx </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2183:    TEAMTALKDLL_API TTBOOL TT_StopRecordingMuxedAudioFileEx(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\backend.rs:240:        unsafe { ffi::api().TT_StopRecordingMuxedAudioFileEx(ptr, channel_id) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StopStreamingMediaFileToChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1859:<li>Use <a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2" title="Stop streaming media file to channel.">TT_StopStreamingMediaFileToChannel()</a> instead.</li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2018:<li><a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2" title="Stop streaming media file to channel.">TT_StopStreamingMediaFileToChannel()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:679:<li>TT_StopStreamingMediaFileToChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:769:<li>TT_StopStreamingMediaFileToChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:183:<tr class="memitem:gaa6b250f5f02f70ab35943b21374cebf2"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2">TT_StopStreamingMediaFileToChannel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:208:<p>To stream a media file to a channel call <a class="el" href="group__mediastream.html#gad58523c65de4dfc2fc0e8beca845a03c" title="Stream media file to channel, e.g. avi-, wav- or MP3-file.">TT_StartStreamingMediaFileToChannel()</a> and to stop the stream call <a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2" title="Stop streaming media file to channel.">TT_StopStreamingMediaFileToChannel()</a>. The user receiving the media stream can control volume levels by calling <a class="el" href="group__sounddevices.html#gab1826616267c007816091ec4f24d0838" title="Set the volume of a user.">TT_SetUserVolume()</a> and <a class="el" href="group__mediastream.html#gab236763cba33f650ded61d2efe880fe3" title="Extract a user&#39;s media video frame for display.">TT_AcquireUserMediaVideoFrame()</a> to obtain video frames.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:509:<a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2" title="Stop streaming media file to channel.">TT_StopStreamingMediaFileToChannel()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:559:<a class="el" href="group__mediastream.html#gaa6b250f5f02f70ab35943b21374cebf2" title="Stop streaming media file to channel.">TT_StopStreamingMediaFileToChannel()</a> </dd></dl>
... (9 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2258:    TEAMTALKDLL_API TTBOOL TT_StopStreamingMediaFileToChannel(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:77:        unsafe { ffi::api().TT_StopStreamingMediaFileToChannel(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_StopVideoCaptureTransmission`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:1646:<li><a class="el" href="group__transmission.html#ga08f7db7429badfe125a6420cb9995aee" title="Stop transmitting from video capture device.">TT_StopVideoCaptureTransmission()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:772:<li>TT_StopVideoCaptureTransmission()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:682:<li>TT_StopVideoCaptureTransmission()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.js:31:    [ "TT_StopVideoCaptureTransmission", "group__transmission.html#ga08f7db7429badfe125a6420cb9995aee", null ]
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:168:<tr class="memitem:ga08f7db7429badfe125a6420cb9995aee"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__transmission.html#ga08f7db7429badfe125a6420cb9995aee">TT_StopVideoCaptureTransmission</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:830:<p>To stop transmitting call <a class="el" href="group__transmission.html#ga08f7db7429badfe125a6420cb9995aee" title="Stop transmitting from video capture device.">TT_StopVideoCaptureTransmission()</a></p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:848:<h2 class="memtitle"><span class="permalink"><a href="#ga08f7db7429badfe125a6420cb9995aee">&#9670;&nbsp;</a></span>TT_StopVideoCaptureTransmission()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__transmission.html:854:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_StopVideoCaptureTransmission </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2191:    TEAMTALKDLL_API TTBOOL TT_StopVideoCaptureTransmission(IN TTInstance* lpTTInstance);
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\video.rs:63:        unsafe { ffi::api().TT_StopVideoCaptureTransmission(self.ptr.0) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_SwapTeamTalkHWND`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2385:<p>When using <a class="el" href="group__initclient.html#gaea369735ecf5c6c75f5a30944f389bbe" title="Create a new TeamTalk client instance where events are posted to a HWND.">TT_InitTeamTalk()</a> a <code>HWND</code> is passed which is used for event handling. If at some point another <code>HWND</code> should be used for event handling this <code>HWND</code> can be swapped using <a class="el" href="group__initclient.html#ga5747b70f13343bfec8764183a2b49f63" title="Replace the HWND passed as parameter to TT_InitTeamTalk with this HWND.">TT_SwapTeamTalkHWND()</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:685:<li>TT_SwapTeamTalkHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:778:<li>TT_SwapTeamTalkHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.js:30:    [ "TT_SwapTeamTalkHWND", "group__initclient.html#ga5747b70f13343bfec8764183a2b49f63", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:150:<tr class="memitem:ga5747b70f13343bfec8764183a2b49f63"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__initclient.html#ga5747b70f13343bfec8764183a2b49f63">TT_SwapTeamTalkHWND</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN HWND hWnd)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:372:<h2 class="memtitle"><span class="permalink"><a href="#ga5747b70f13343bfec8764183a2b49f63">&#9670;&nbsp;</a></span>TT_SwapTeamTalkHWND()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__initclient.html:378:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_SwapTeamTalkHWND </td>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\search\all_13.js:242:  ['tt_5fswapteamtalkhwnd_906',['TT_SwapTeamTalkHWND',['../group__initclient.html#ga5747b70f13343bfec8764183a2b49f63',1,'TeamTalk.h']]],
... (5 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:1968:    TEAMTALKDLL_API TTBOOL TT_SwapTeamTalkHWND(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\core.rs:135:        unsafe { ffi::api().TT_SwapTeamTalkHWND(self.ptr.0, hwnd) == 1 }
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_UpdateLocalPlayback`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:746:<p>While the media file is playing it's possible to use <a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback()</a> to change the media stream's properties. The progress of the media file playback can be monitor through <a class="el" href="group__events.html#gga7c228530d18e96b483502c824c700224aa78460c3432a5aa5f0a66a7ee2021922" title="Media file played locally is processing.">CLIENTEVENT_LOCAL_MEDIAFILE</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:817:<li>New functions <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a>, <a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback()</a> and <a class="el" href="group__mediastream.html#ga65ca66b1ee8b9b907e489c3dfd3fda49">TT_StopLocalPlayback()</a> for local media playback.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:688:<li>TT_UpdateLocalPlayback()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:796:<li>TT_UpdateLocalPlayback()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:189:<tr class="memitem:ga339398e483abcbc3f9b7fea989f509aa"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN INT32 nPlaybackSessionID, IN const <a class="el" href="struct_media_file_playback.html">MediaFilePlayback</a> *lpMediaFilePlayback)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:222:<p>Specify this value as uOffsetMSec in <a class="el" href="struct_media_file_playback.html" title="Properties for initializing or updating a file for media streaming.">MediaFilePlayback</a> when calling <a class="el" href="group__mediastream.html#ga02910d5b44042ed667f4f73bacbea1e4" title="Play media file using settings from TTInstance.">TT_InitLocalPlayback()</a> and <a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback()</a> to ignore rewind or forward. </p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:314:<a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:631:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mediastream.html#ga339398e483abcbc3f9b7fea989f509aa">TT_UpdateLocalPlayback()</a> </dd>
... (13 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2266:    TEAMTALKDLL_API TTBOOL TT_UpdateLocalPlayback(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:90:            ffi::api().TT_UpdateLocalPlayback(self.ptr.0, session_id, &playback.to_ffi()) == 1
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_UpdateStreamingMediaFileToChannel`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:753:<p>It's also possible to change the offset and pause the media file using <a class="el" href="group__mediastream.html#ga670872af8760ce17049cb7d15bdf6da7" title="Update active media file being streamed to channel.">TT_UpdateStreamingMediaFileToChannel()</a> and <a class="el" href="struct_media_file_playback.html" title="Properties for initializing or updating a file for media streaming.">MediaFilePlayback</a>.</p>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:845:<li>New function <a class="el" href="group__mediastream.html#ga670872af8760ce17049cb7d15bdf6da7" title="Update active media file being streamed to channel.">TT_UpdateStreamingMediaFileToChannel()</a> for updating an active media file.<ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:799:<li>TT_UpdateStreamingMediaFileToChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:691:<li>TT_UpdateStreamingMediaFileToChannel()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:180:<tr class="memitem:ga670872af8760ce17049cb7d15bdf6da7"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__mediastream.html#ga670872af8760ce17049cb7d15bdf6da7">TT_UpdateStreamingMediaFileToChannel</a> (IN <a class="el" href="group__initclient.html#ga3c34b9935bd1f63aa90c94cf9639f1c0">TTInstance</a> *lpTTInstance, IN const <a class="el" href="struct_media_file_playback.html">MediaFilePlayback</a> *lpMediaFilePlayback, IN const <a class="el" href="struct_video_codec.html">VideoCodec</a> *lpVideoCodec)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:318:<a class="el" href="group__mediastream.html#ga670872af8760ce17049cb7d15bdf6da7" title="Update active media file being streamed to channel.">TT_UpdateStreamingMediaFileToChannel()</a> </dd></dl>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:507:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__mediastream.html#ga670872af8760ce17049cb7d15bdf6da7" title="Update active media file being streamed to channel.">TT_UpdateStreamingMediaFileToChannel()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__mediastream.html:516:<h2 class="memtitle"><span class="permalink"><a href="#ga670872af8760ce17049cb7d15bdf6da7">&#9670;&nbsp;</a></span>TT_UpdateStreamingMediaFileToChannel()</h2>
... (12 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2253:    TEAMTALKDLL_API TTBOOL TT_UpdateStreamingMediaFileToChannel(IN TTInstance* lpTTInstance,
```

### crates/teamtalk/src
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk\src\client\media.rs:67:            ffi::api().TT_UpdateStreamingMediaFileToChannel(
```

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Windows_GetDesktopActiveHWND`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2130:<li><a class="el" href="group__desktopshare.html#ga2c70fbe096c8405273e265a1ca446cb4" title="Get the handle (HWND) of the window which is currently active (focused) on the Windows desktop.">TT_Windows_GetDesktopActiveHWND()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:694:<li>TT_Windows_GetDesktopActiveHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:808:<li>TT_Windows_GetDesktopActiveHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:70:    [ "TT_Windows_GetDesktopActiveHWND", "group__desktopshare.html#ga2c70fbe096c8405273e265a1ca446cb4", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:200:<tr class="memitem:ga2c70fbe096c8405273e265a1ca446cb4"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> HWND&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga2c70fbe096c8405273e265a1ca446cb4">TT_Windows_GetDesktopActiveHWND</a> (void)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:282:<li><a class="el" href="group__desktopshare.html#ga2c70fbe096c8405273e265a1ca446cb4" title="Get the handle (HWND) of the window which is currently active (focused) on the Windows desktop.">TT_Windows_GetDesktopActiveHWND()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:941:<h2 class="memtitle"><span class="permalink"><a href="#ga2c70fbe096c8405273e265a1ca446cb4">&#9670;&nbsp;</a></span>TT_Windows_GetDesktopActiveHWND()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:947:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> HWND TT_Windows_GetDesktopActiveHWND </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2303:    TEAMTALKDLL_API HWND TT_Windows_GetDesktopActiveHWND(void);
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Windows_GetDesktopHWND`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2131:<li><a class="el" href="group__desktopshare.html#gadf254a23d13f415ae8416bad463ed347" title="Get the handle (HWND) of the Windows desktop (full desktop).">TT_Windows_GetDesktopHWND()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:697:<li>TT_Windows_GetDesktopHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:811:<li>TT_Windows_GetDesktopHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:71:    [ "TT_Windows_GetDesktopHWND", "group__desktopshare.html#gadf254a23d13f415ae8416bad463ed347", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:203:<tr class="memitem:gadf254a23d13f415ae8416bad463ed347"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> HWND&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#gadf254a23d13f415ae8416bad463ed347">TT_Windows_GetDesktopHWND</a> (void)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:286:<li><a class="el" href="group__desktopshare.html#gadf254a23d13f415ae8416bad463ed347" title="Get the handle (HWND) of the Windows desktop (full desktop).">TT_Windows_GetDesktopHWND()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:961:<h2 class="memtitle"><span class="permalink"><a href="#gadf254a23d13f415ae8416bad463ed347">&#9670;&nbsp;</a></span>TT_Windows_GetDesktopHWND()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:967:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> HWND TT_Windows_GetDesktopHWND </td>
... (6 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2306:    TEAMTALKDLL_API HWND TT_Windows_GetDesktopHWND(void);
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Windows_GetDesktopWindowHWND`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2132:<li><a class="el" href="group__desktopshare.html#ga27b7799cf5d2ea3673ca02fb929e9827" title="Enumerate all the handles (HWND) of visible windows. Increment nIndex until the function returns FALS...">TT_Windows_GetDesktopWindowHWND()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:700:<li>TT_Windows_GetDesktopWindowHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:814:<li>TT_Windows_GetDesktopWindowHWND()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:206:<tr class="memitem:ga27b7799cf5d2ea3673ca02fb929e9827"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga27b7799cf5d2ea3673ca02fb929e9827">TT_Windows_GetDesktopWindowHWND</a> (IN INT32 nIndex, OUT HWND *lpHWnd)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:290:<li><a class="el" href="group__desktopshare.html#ga27b7799cf5d2ea3673ca02fb929e9827" title="Enumerate all the handles (HWND) of visible windows. Increment nIndex until the function returns FALS...">TT_Windows_GetDesktopWindowHWND()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:677:<dl class="section see"><dt>See also</dt><dd><a class="el" href="group__desktopshare.html#ga27b7799cf5d2ea3673ca02fb929e9827" title="Enumerate all the handles (HWND) of visible windows. Increment nIndex until the function returns FALS...">TT_Windows_GetDesktopWindowHWND()</a> </dd>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:981:<h2 class="memtitle"><span class="permalink"><a href="#ga27b7799cf5d2ea3673ca02fb929e9827">&#9670;&nbsp;</a></span>TT_Windows_GetDesktopWindowHWND()</h2>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:987:          <td class="memname"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a> TT_Windows_GetDesktopWindowHWND </td>
... (8 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2309:    TEAMTALKDLL_API TTBOOL TT_Windows_GetDesktopWindowHWND(IN INT32 nIndex,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## `TT_Windows_GetWindow`
### TEAMTALK_DLL/Documentation/C-API
```text
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\changelog.html:2133:<li><a class="el" href="group__desktopshare.html#ga82ec8fe664c3ff00d1c760a8ef6dda2b" title="Get the properties of a window from its window handle (HWND).">TT_Windows_GetWindow()</a></li>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_func.html:703:<li>TT_Windows_GetWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\globals_t.html:817:<li>TT_Windows_GetWindow()
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.js:73:    [ "TT_Windows_GetWindow", "group__desktopshare.html#ga82ec8fe664c3ff00d1c760a8ef6dda2b", null ],
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:207:<tr class="memdesc:ga27b7799cf5d2ea3673ca02fb929e9827"><td class="mdescLeft">&#160;</td><td class="mdescRight">Enumerate all the handles (<code>HWND</code>) of visible windows. Increment <code>nIndex</code> until the function returns FALSE. Use <a class="el" href="group__desktopshare.html#ga82ec8fe664c3ff00d1c760a8ef6dda2b" title="Get the properties of a window from its window handle (HWND).">TT_Windows_GetWindow()</a> to get information about each window.  <a href="group__desktopshare.html#ga27b7799cf5d2ea3673ca02fb929e9827">More...</a><br /></td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:209:<tr class="memitem:ga82ec8fe664c3ff00d1c760a8ef6dda2b"><td class="memItemLeft" align="right" valign="top"><a class="el" href="_team_talk_8h.html#a3849440832783229a1eadc6c4fc08608">TEAMTALKDLL_API</a> <a class="el" href="_team_talk_8h.html#adeacdb26d685cce3a89464597c2bc0b9">TTBOOL</a>&#160;</td><td class="memItemRight" valign="bottom"><a class="el" href="group__desktopshare.html#ga82ec8fe664c3ff00d1c760a8ef6dda2b">TT_Windows_GetWindow</a> (IN HWND hWnd, OUT <a class="el" href="struct_share_window.html">ShareWindow</a> *lpShareWindow)</td></tr>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:294:<li><a class="el" href="group__desktopshare.html#ga82ec8fe664c3ff00d1c760a8ef6dda2b" title="Get the properties of a window from its window handle (HWND).">TT_Windows_GetWindow()</a><ul>
D:\downloads\repos\TeamTalkRust\TEAMTALK_DLL\Documentation\C-API\group__desktopshare.html:679:<a class="el" href="group__desktopshare.html#ga82ec8fe664c3ff00d1c760a8ef6dda2b" title="Get the properties of a window from its window handle (HWND).">TT_Windows_GetWindow()</a></dd>
... (11 more)
```

### crates/teamtalk-sys
```text
D:\downloads\repos\TeamTalkRust\crates\teamtalk-sys\TeamTalk.h:2330:    TEAMTALKDLL_API TTBOOL TT_Windows_GetWindow(IN HWND hWnd,
```

### crates/teamtalk/src
No matches

### crates/teamtalk/tests
No matches

### docs
No matches

### README.md
No matches

## Coverage Summary

- Symbols without wrapper/sys mapping: 0
- Symbols without direct tests reference: 202
- List: TT_AcquireUserAudioBlock, TT_AcquireUserDesktopWindow, TT_AcquireUserDesktopWindowEx, TT_AcquireUserMediaVideoFrame, TT_AcquireUserVideoCaptureFrame, TT_AutoPositionUsers, TT_CancelFileTransfer, TT_CloseDesktopWindow, TT_CloseSoundDuplexDevices, TT_CloseSoundInputDevice, TT_CloseSoundLoopbackTest, TT_CloseSoundOutputDevice, TT_CloseTeamTalk, TT_CloseVideoCaptureDevice, TT_Connect, TT_ConnectEx, TT_ConnectSysID, TT_DBG_GETDATAPTR, TT_DBG_SIZEOF, TT_DBG_SetSoundInputTone, TT_DBG_WriteAudioFileTone, TT_DesktopInput_Execute, TT_DesktopInput_KeyTranslate, TT_Disconnect, TT_DoBan, TT_DoBanIPAddress, TT_DoBanUser, TT_DoBanUserEx, TT_DoChangeNickname, TT_DoChangeStatus, TT_DoChannelOp, TT_DoChannelOpEx, TT_DoDeleteFile, TT_DoDeleteUserAccount, TT_DoJoinChannel, TT_DoJoinChannelByID, TT_DoKickUser, TT_DoLeaveChannel, TT_DoListBans, TT_DoListUserAccounts, TT_DoLogin, TT_DoLoginEx, TT_DoLogout, TT_DoMakeChannel, TT_DoMoveUser, TT_DoNewUserAccount, TT_DoQueryServerStats, TT_DoQuit, TT_DoRecvFile, TT_DoRemoveChannel, TT_DoSaveConfig, TT_DoSendFile, TT_DoSubscribe, TT_DoTextMessage, TT_DoUnBanUser, TT_DoUnBanUserEx, TT_DoUnsubscribe, TT_DoUpdateChannel, TT_DoUpdateServer, TT_Enable3DSoundPositioning, TT_EnableAudioBlockEvent, TT_EnableAudioBlockEventEx, TT_EnableVoiceActivation, TT_EnableVoiceTransmission, TT_Firewall_AddAppException, TT_Firewall_AppExceptionExists, TT_Firewall_Enable, TT_Firewall_IsEnabled, TT_Firewall_RemoveAppException, TT_GetChannel, TT_GetChannelFile, TT_GetChannelFiles, TT_GetChannelIDFromPath, TT_GetChannelPath, TT_GetChannelUsers, TT_GetClientKeepAlive, TT_GetClientStatistics, TT_GetDefaultSoundDevices, TT_GetDefaultSoundDevicesEx, TT_GetErrorMessage, TT_GetFileTransferInfo, TT_GetFlags, TT_GetMediaFileInfo, TT_GetMyChannelID, TT_GetMyUserAccount, TT_GetMyUserData, TT_GetMyUserID, TT_GetMyUserRights, TT_GetMyUserType, TT_GetRootChannelID, TT_GetServerChannels, TT_GetServerProperties, TT_GetServerUsers, TT_GetSoundDeviceEffects, TT_GetSoundDevices, TT_GetSoundInputGainLevel, TT_GetSoundInputLevel, TT_GetSoundInputPreprocess, TT_GetSoundInputPreprocessEx, TT_GetSoundOutputVolume, TT_GetUser, TT_GetUserByUsername, TT_GetUserJitterControl, TT_GetUserStatistics, TT_GetVideoCaptureDevices, TT_GetVoiceActivationLevel, TT_GetVoiceActivationStopDelay, TT_HotKey_GetKeyString, TT_HotKey_InstallTestHook, TT_HotKey_IsActive, TT_HotKey_Register, TT_HotKey_RemoveTestHook, TT_HotKey_Unregister, TT_InitLocalPlayback, TT_InitSoundDuplexDevices, TT_InitSoundInputDevice, TT_InitSoundInputSharedDevice, TT_InitSoundOutputDevice, TT_InitSoundOutputSharedDevice, TT_InitVideoCaptureDevice
- Test heuristic: raw symbol match OR mapped wrapper method match in tests.

## Next Actions
- Convert uncovered symbols into plan.md findings with disposition.
- Implement wrappers/tests/docs or mark explicit defer reasons.
