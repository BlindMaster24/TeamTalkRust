#if !defined(TEAMTALKDLL_H)
#define TEAMTALKDLL_H





#define TEAMTALK_VERSION "5.19.0.5170"


#if defined(WIN32)

#ifdef TEAMTALKDLL_EXPORTS
#define TEAMTALKDLL_API __declspec(dllexport)
#else
#define TEAMTALKDLL_API __declspec(dllimport)
#endif

#else 

#define TEAMTALKDLL_API

#endif


#ifdef __cplusplus
extern "C" {
#endif

    
#if !defined(TEAMTALK_TYPES)

#define TEAMTALK_TYPES 1

#if defined(WIN32)
    

    
    typedef WCHAR TTCHAR;
    typedef int TTBOOL;
#else
    typedef char TTCHAR;
    typedef int TTBOOL;
    typedef unsigned short UINT16;
    typedef int INT32;
    typedef long long INT64;
    typedef unsigned int UINT32;
    typedef void VOID;



#define IN
#define OUT

#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

#endif

    
#define TT_STRLEN 512

    
#define TT_USERID_MAX 0xFFF

    
#define TT_CHANNELID_MAX 0xFFF

    
#define TT_VIDEOFORMATS_MAX 1024

    
#define TT_TRANSMITUSERS_MAX 128

    
#define TT_CLASSROOM_FREEFORALL 0xFFF

     
#define TT_CLASSROOM_USERID_INDEX 0

      
#define TT_CLASSROOM_STREAMTYPE_INDEX 1

    
#define TT_TRANSMITUSERS_FREEFORALL 0xFFF

    
#define TT_TRANSMITUSERS_USERID_INDEX 0

     
#define TT_TRANSMITUSERS_STREAMTYPE_INDEX 1

    
#define TT_CHANNELS_OPERATOR_MAX 16

    
#define TT_TRANSMITQUEUE_MAX 16

    
#define TT_SAMPLERATES_MAX 16


#define TT_DESKTOPINPUT_MAX 16


#define TT_DESKTOPINPUT_KEYCODE_IGNORE  0xFFFFFFFF


#define TT_DESKTOPINPUT_MOUSEPOS_IGNORE 0xFFFF


#define TT_DESKTOPINPUT_KEYCODE_LMOUSEBTN 0x1000


#define TT_DESKTOPINPUT_KEYCODE_RMOUSEBTN 0x1001


#define TT_DESKTOPINPUT_KEYCODE_MMOUSEBTN 0x1002


#define TT_MEDIAPLAYBACK_OFFSET_IGNORE 0xFFFFFFFF

#endif 

    

    
    typedef enum StreamType
    {
        
        STREAMTYPE_NONE                     = 0x00000000,
        
        STREAMTYPE_VOICE                    = 0x00000001,
        
        STREAMTYPE_VIDEOCAPTURE             = 0x00000002,
        
        STREAMTYPE_MEDIAFILE_AUDIO          = 0x00000004,
        
        STREAMTYPE_MEDIAFILE_VIDEO          = 0x00000008,
        
        STREAMTYPE_DESKTOP                  = 0x00000010,
        
        STREAMTYPE_DESKTOPINPUT             = 0x00000020,
        
        STREAMTYPE_MEDIAFILE                = STREAMTYPE_MEDIAFILE_AUDIO |
                                              STREAMTYPE_MEDIAFILE_VIDEO,
        
        STREAMTYPE_CHANNELMSG               = 0x00000040,
        
        STREAMTYPE_LOCALMEDIAPLAYBACK_AUDIO = 0x00000080,

        
        STREAMTYPE_CLASSROOM_ALL            = STREAMTYPE_VOICE |
                                              STREAMTYPE_VIDEOCAPTURE |
                                              STREAMTYPE_DESKTOP |
                                              STREAMTYPE_MEDIAFILE |
                                              STREAMTYPE_CHANNELMSG,
    } StreamType;

    
    typedef UINT32 StreamTypes;

    

    

    
    typedef enum SoundSystem
    {
        
        SOUNDSYSTEM_NONE = 0,
        
        SOUNDSYSTEM_WINMM = 1,
        
        SOUNDSYSTEM_DSOUND = 2,
        
        SOUNDSYSTEM_ALSA = 3,
        
        SOUNDSYSTEM_COREAUDIO = 4,
        
        SOUNDSYSTEM_WASAPI = 5,
        
        SOUNDSYSTEM_OPENSLES_ANDROID = 7,
        
        SOUNDSYSTEM_AUDIOUNIT = 8,
        
        SOUNDSYSTEM_AUDIOUNIT_IOS = SOUNDSYSTEM_AUDIOUNIT,
        
        SOUNDSYSTEM_PULSEAUDIO = 10,
    } SoundSystem;

    
    typedef enum SoundDeviceFeature
    {
        
        SOUNDDEVICEFEATURE_NONE             = 0x0000,
        
        SOUNDDEVICEFEATURE_AEC              = 0x0001,
        
        SOUNDDEVICEFEATURE_AGC              = 0x0002,
        
        SOUNDDEVICEFEATURE_DENOISE          = 0x0004,
        
        SOUNDDEVICEFEATURE_3DPOSITION       = 0x0008,
        
        SOUNDDEVICEFEATURE_DUPLEXMODE       = 0x0010,
        
        SOUNDDEVICEFEATURE_DEFAULTCOMDEVICE = 0x0020,
    } SoundDeviceFeature;

    
    typedef UINT32 SoundDeviceFeatures;

    
    typedef struct SoundDevice
    {
        
        INT32 nDeviceID;
        
        SoundSystem nSoundSystem;
        
        TTCHAR szDeviceName[TT_STRLEN];
        
        TTCHAR szDeviceID[TT_STRLEN];
        
        INT32 nWaveDeviceID;
        
        TTBOOL bSupports3D;
        
        INT32 nMaxInputChannels;
        
        INT32 nMaxOutputChannels;
        
        INT32 inputSampleRates[TT_SAMPLERATES_MAX];
        
        INT32 outputSampleRates[TT_SAMPLERATES_MAX];
        
        INT32 nDefaultSampleRate;
        
        SoundDeviceFeatures uSoundDeviceFeatures;
    } SoundDevice;

    
    typedef struct SoundDeviceEffects
    {
        
        TTBOOL bEnableAGC;
        
        TTBOOL bEnableDenoise;
        
        TTBOOL bEnableEchoCancellation;
    } SoundDeviceEffects;


#define TT_SOUNDDEVICE_ID_SHARED_FLAG           0x00000800


#define TT_SOUNDDEVICE_ID_MASK                  0x000007FF
    

#define TT_SOUNDDEVICE_ID_REMOTEIO              0
    

#define TT_SOUNDDEVICE_ID_VOICEPREPROCESSINGIO  (1 | TT_SOUNDDEVICE_ID_SHARED_FLAG)
    

#define TT_SOUNDDEVICE_ID_OPENSLES_DEFAULT      0


#define TT_SOUNDDEVICE_ID_OPENSLES_VOICECOM     1
    

#define TT_SOUNDDEVICE_ID_TEAMTALK_VIRTUAL      1978

    
    typedef enum SoundLevel
    {
        
        SOUND_VU_MAX = 100,
        
        SOUND_VU_MIN = 0,
        
        SOUND_VOLUME_MAX = 32000,
        
        SOUND_VOLUME_DEFAULT = 1000,
        
        SOUND_VOLUME_MIN = 0,
        
        SOUND_GAIN_MAX = 32000,
        
        SOUND_GAIN_DEFAULT = 1000,
        
        SOUND_GAIN_MIN = 0
    } SoundLevel;

    
    typedef struct AudioBlock
    {
        
        INT32 nStreamID;
        
        INT32 nSampleRate;
        
        INT32 nChannels;
        
        VOID* lpRawAudio;
        
        INT32 nSamples;
        
        UINT32 uSampleIndex;
        
        StreamTypes uStreamTypes;
    } AudioBlock;


#define TT_LOCAL_USERID 0


#define TT_LOCAL_TX_USERID 0x1002


#define TT_MUXED_USERID 0x1001 

    

    

    
    typedef enum MediaFileStatus
    {
        MFS_CLOSED          = 0,
        
        MFS_ERROR           = 1,
        
        MFS_STARTED         = 2,
        
        MFS_FINISHED        = 3,
        
        MFS_ABORTED         = 4,
        
        MFS_PAUSED          = 5,
        
        MFS_PLAYING         = 6
    } MediaFileStatus;

    
    typedef enum AudioFileFormat
    {
        
        AFF_NONE                 = 0,
        
        AFF_CHANNELCODEC_FORMAT  = 1,
        
        AFF_WAVE_FORMAT          = 2,
        
        AFF_MP3_16KBIT_FORMAT    = 3,
        
        AFF_MP3_32KBIT_FORMAT    = 4,
        
        AFF_MP3_64KBIT_FORMAT    = 5,
        
        AFF_MP3_128KBIT_FORMAT   = 6,
        
        AFF_MP3_256KBIT_FORMAT   = 7,
        
        AFF_MP3_320KBIT_FORMAT   = 8,
    } AudioFileFormat;

    
    typedef struct AudioFormat
    {
        
        AudioFileFormat nAudioFmt;
        
        INT32 nSampleRate;
        
        INT32 nChannels;
    } AudioFormat;

    

    

    
    typedef enum FourCC
    {
        
        FOURCC_NONE   =   0,
        
        FOURCC_I420   = 100,
        
        FOURCC_YUY2   = 101,
        
        FOURCC_RGB32  = 102
    } FourCC;

    
    typedef struct VideoFormat
    {
        
        INT32 nWidth;
        
        INT32 nHeight;
        
        INT32 nFPS_Numerator; 
        
        INT32 nFPS_Denominator; 
        
        FourCC picFourCC; 
    } VideoFormat;

    
    typedef struct VideoFrame
    {
        
        INT32 nWidth;
        
        INT32 nHeight;
        
        INT32 nStreamID;
        
        TTBOOL bKeyFrame;
        
        VOID* frameBuffer;
        
        INT32 nFrameBufferSize;
    } VideoFrame;

    
    typedef struct VideoCaptureDevice
    {
        
        TTCHAR szDeviceID[TT_STRLEN];
        
        TTCHAR szDeviceName[TT_STRLEN];
         
        TTCHAR szCaptureAPI[TT_STRLEN];
        
        VideoFormat videoFormats[TT_VIDEOFORMATS_MAX];
        
        INT32 nVideoFormatsCount; 
    } VideoCaptureDevice;

    

    

    
    typedef enum BitmapFormat
    {
        
        BMP_NONE            = 0,
        
        BMP_RGB8_PALETTE    = 1,
        
        BMP_RGB16_555       = 2,
        
        BMP_RGB24           = 3,
        
        BMP_RGB32           = 4
    } BitmapFormat;

    
    typedef enum DesktopProtocol
    {
        
        DESKTOPPROTOCOL_ZLIB_1  = 1
    } DesktopProtocol;

    
    typedef struct DesktopWindow
    {
        
        INT32 nWidth;
        
        INT32 nHeight;
        
        BitmapFormat bmpFormat;
        
        INT32 nBytesPerLine;
        
        INT32 nSessionID;
        
        DesktopProtocol nProtocol;
        
        VOID* frameBuffer;
        
        INT32 nFrameBufferSize;
    } DesktopWindow;

    
    typedef enum DesktopKeyState
    {
        
        DESKTOPKEYSTATE_NONE       = 0x00000000,
        
        DESKTOPKEYSTATE_DOWN       = 0x00000001,
        
        DESKTOPKEYSTATE_UP         = 0x00000002,
    } DesktopKeyState;

    
    typedef UINT32 DesktopKeyStates;

    
    typedef struct DesktopInput
    {
        
        UINT16 uMousePosX;
        
        UINT16 uMousePosY;
        
        UINT32 uKeyCode;
        
        DesktopKeyStates uKeyState;
    } DesktopInput;

    

    

    
    typedef struct SpeexCodec
    {
        
        INT32 nBandmode;
        
        INT32 nQuality;
        
        INT32 nTxIntervalMSec;
        
        TTBOOL bStereoPlayback;
    } SpeexCodec;

    
    typedef struct SpeexVBRCodec
    {
        
        INT32 nBandmode;
        
        INT32 nQuality;
        
        INT32 nBitRate;
        
        INT32 nMaxBitRate;
        
        TTBOOL bDTX;
        
        INT32 nTxIntervalMSec;
        
        TTBOOL bStereoPlayback; 
    } SpeexVBRCodec;


#define SPEEX_NB_MIN_BITRATE 2150

#define SPEEX_NB_MAX_BITRATE 24600

#define SPEEX_WB_MIN_BITRATE 3950

#define SPEEX_WB_MAX_BITRATE 42200

#define SPEEX_UWB_MIN_BITRATE 4150

#define SPEEX_UWB_MAX_BITRATE 44000

    
    typedef struct OpusCodec
    {
        
        INT32 nSampleRate;
        
        INT32 nChannels;
        
        INT32 nApplication;
        
        INT32 nComplexity;
        
        TTBOOL bFEC;
        
        TTBOOL bDTX;
        
        INT32 nBitRate;
        
        TTBOOL bVBR;
        
        TTBOOL bVBRConstraint;
        
        INT32 nTxIntervalMSec;
        
        INT32 nFrameSizeMSec;
    } OpusCodec;


#define OPUS_APPLICATION_VOIP 2048

#define OPUS_APPLICATION_AUDIO 2049

#define OPUS_MIN_BITRATE 6000

#define OPUS_MAX_BITRATE 510000

#define OPUS_MIN_FRAMESIZE 2 

#define OPUS_MAX_FRAMESIZE 60

#define OPUS_REALMAX_FRAMESIZE 120

    
    typedef struct SpeexDSP
    {
        
        TTBOOL bEnableAGC;
        
        INT32 nGainLevel;
        
        INT32 nMaxIncDBSec;
        
        INT32 nMaxDecDBSec;
        
        INT32 nMaxGainDB;
        
        TTBOOL bEnableDenoise;
        
        INT32 nMaxNoiseSuppressDB;
        
        TTBOOL bEnableEchoCancellation;
        
        INT32 nEchoSuppress;
        
        INT32 nEchoSuppressActive;
    } SpeexDSP;

    
    typedef struct TTAudioPreprocessor
    {
        
        INT32 nGainLevel;
        
        TTBOOL bMuteLeftSpeaker;
        
        TTBOOL bMuteRightSpeaker;
    } TTAudioPreprocessor;

    
    typedef struct WebRTCAudioPreprocessor
    {
        
        struct
        {
            
            TTBOOL bEnable;
            
            float fFixedGainFactor;
        } preamplifier;
        
        struct
        {
            
            TTBOOL bEnable;
        } echocanceller;
        
        struct
        {
            
            TTBOOL bEnable;
            
            INT32 nLevel;
        } noisesuppression;
        
        struct
        {
            
            TTBOOL bEnable;
            
            struct
            {
                
                float fGainDB;
            } fixeddigital;
            
            struct
            {
                
                TTBOOL bEnable;
                
                float fHeadRoomDB;
                
                float fMaxGainDB;
                
                float fInitialGainDB;
                
                float fMaxGainChangeDBPerSecond;
                
                float fMaxOutputNoiseLevelDBFS;
            } adaptivedigital;
        } gaincontroller2;
    } WebRTCAudioPreprocessor;


#define WEBRTC_GAINCONTROLLER2_FIXEDGAIN_MAX 49.9f

    
    typedef enum AudioPreprocessorType
    {
        
        NO_AUDIOPREPROCESSOR        = 0,
        
        SPEEXDSP_AUDIOPREPROCESSOR  = 1,
        
        TEAMTALK_AUDIOPREPROCESSOR  = 2,
        
        WEBRTC_AUDIOPREPROCESSOR_OBSOLETE_R4332    = 3,
        
        WEBRTC_AUDIOPREPROCESSOR    = 4,
    } AudioPreprocessorType;

    
    typedef struct AudioPreprocessor
    {
        
        AudioPreprocessorType nPreprocessor;
        union
        {
            
            SpeexDSP speexdsp;
            
            TTAudioPreprocessor ttpreprocessor;
            
            WebRTCAudioPreprocessor webrtc;
        };
    } AudioPreprocessor;
    
    
    typedef struct WebMVP8Codec
    {
        union
        {
            
            INT32 nRcTargetBitrate;
            
            UINT32 rc_target_bitrate;
        };
        
        UINT32 nEncodeDeadline;
    } WebMVP8Codec;


#define WEBM_VPX_DL_REALTIME 1

#define WEBM_VPX_DL_GOOD_QUALITY 1000000

#define WEBM_VPX_DL_BEST_QUALITY 0

    
    typedef enum Codec
    {
        
        NO_CODEC                    = 0,
        
        SPEEX_CODEC                 = 1,
        
        SPEEX_VBR_CODEC             = 2,
        
        OPUS_CODEC                  = 3,
        
        WEBM_VP8_CODEC              = 128,
    } Codec;

    
    typedef struct AudioCodec
    {
        
        Codec nCodec;  
        union
        {
            
            SpeexCodec speex;
            
            SpeexVBRCodec speex_vbr;
            
            OpusCodec opus;
        };
    } AudioCodec;

    
    typedef struct AudioConfig
    {
        
        TTBOOL bEnableAGC;
        
        INT32 nGainLevel;
    } AudioConfig;

    
    typedef struct VideoCodec
    {
        
        Codec nCodec;  
        union
        {
            WebMVP8Codec webm_vp8;
        };
    } VideoCodec;
    

    

    
    typedef struct MediaFileInfo
    {
        
        MediaFileStatus nStatus;
        
        TTCHAR szFileName[TT_STRLEN];
        
        AudioFormat audioFmt;
        
        VideoFormat videoFmt;
        
        UINT32 uDurationMSec;
        
        UINT32 uElapsedMSec;
    } MediaFileInfo;

    
    typedef struct MediaFilePlayback
    {
        
        UINT32 uOffsetMSec;
        
        TTBOOL bPaused;
        
        AudioPreprocessor audioPreprocessor;
    } MediaFilePlayback;

    
    typedef struct AudioInputProgress
    {
        
        INT32 nStreamID;
        
        UINT32 uQueueMSec;
        
        UINT32 uElapsedMSec;
    } AudioInputProgress;

    

    

    
    typedef enum UserRight
    {
        
        USERRIGHT_NONE                      = 0x00000000, 
        
        USERRIGHT_MULTI_LOGIN               = 0x00000001,
        
        USERRIGHT_VIEW_ALL_USERS            = 0x00000002,
         
        USERRIGHT_CREATE_TEMPORARY_CHANNEL  = 0x00000004,
         
        USERRIGHT_MODIFY_CHANNELS           = 0x00000008,
        
        USERRIGHT_TEXTMESSAGE_BROADCAST     = 0x00000010,
        
        USERRIGHT_KICK_USERS                = 0x00000020,
        
        USERRIGHT_BAN_USERS                 = 0x00000040,
        
        USERRIGHT_MOVE_USERS                = 0x00000080,
        
        USERRIGHT_OPERATOR_ENABLE           = 0x00000100,
        
        USERRIGHT_UPLOAD_FILES              = 0x00000200,
        
        USERRIGHT_DOWNLOAD_FILES            = 0x00000400,
        
        USERRIGHT_UPDATE_SERVERPROPERTIES   = 0x00000800,
        
        USERRIGHT_TRANSMIT_VOICE            = 0x00001000, 
        
        USERRIGHT_TRANSMIT_VIDEOCAPTURE     = 0x00002000,
        
        USERRIGHT_TRANSMIT_DESKTOP          = 0x00004000,
        
        USERRIGHT_TRANSMIT_DESKTOPINPUT     = 0x00008000,
        
        USERRIGHT_TRANSMIT_MEDIAFILE_AUDIO  = 0x00010000,
        
        USERRIGHT_TRANSMIT_MEDIAFILE_VIDEO  = 0x00020000,
        
        USERRIGHT_TRANSMIT_MEDIAFILE = USERRIGHT_TRANSMIT_MEDIAFILE_VIDEO | USERRIGHT_TRANSMIT_MEDIAFILE_AUDIO,
        
        USERRIGHT_LOCKED_NICKNAME           = 0x00040000,
        
        USERRIGHT_LOCKED_STATUS             = 0x00080000,
        
        USERRIGHT_RECORD_VOICE              = 0x00100000,
        
        USERRIGHT_VIEW_HIDDEN_CHANNELS      = 0x00200000,
        
        USERRIGHT_TEXTMESSAGE_USER          = 0x00400000,
        
        USERRIGHT_TEXTMESSAGE_CHANNEL       = 0x00800000,
    } UserRight;

    
    typedef UINT32 UserRights;


    
    typedef enum ServerLogEvent
    {
        
        SERVERLOGEVENT_NONE                        = 0x00000000,
        
        SERVERLOGEVENT_USER_CONNECTED              = 0x00000001,
        
        SERVERLOGEVENT_USER_DISCONNECTED           = 0x00000002,
        
        SERVERLOGEVENT_USER_LOGGEDIN               = 0x00000004,
        
        SERVERLOGEVENT_USER_LOGGEDOUT              = 0x00000008,
        
        SERVERLOGEVENT_USER_LOGINFAILED            = 0x00000010,
        
        SERVERLOGEVENT_USER_TIMEDOUT               = 0x00000020,
        
        SERVERLOGEVENT_USER_KICKED                 = 0x00000040,
        
        SERVERLOGEVENT_USER_BANNED                 = 0x00000080,
        
        SERVERLOGEVENT_USER_UNBANNED               = 0x00000100,
        
        SERVERLOGEVENT_USER_UPDATED                = 0x00000200,
        
        SERVERLOGEVENT_USER_JOINEDCHANNEL          = 0x00000400,
        
        SERVERLOGEVENT_USER_LEFTCHANNEL            = 0x00000800,
        
        SERVERLOGEVENT_USER_MOVED                  = 0x00001000,
        
        SERVERLOGEVENT_USER_TEXTMESSAGE_PRIVATE    = 0x00002000,
        
        SERVERLOGEVENT_USER_TEXTMESSAGE_CUSTOM     = 0x00004000,
        
        SERVERLOGEVENT_USER_TEXTMESSAGE_CHANNEL    = 0x00008000,
        
        SERVERLOGEVENT_USER_TEXTMESSAGE_BROADCAST  = 0x00010000,
        
        SERVERLOGEVENT_CHANNEL_CREATED             = 0x00020000,
        
        SERVERLOGEVENT_CHANNEL_UPDATED             = 0x00040000,
        
        SERVERLOGEVENT_CHANNEL_REMOVED             = 0x00080000,
        
        SERVERLOGEVENT_FILE_UPLOADED               = 0x00100000,
        
        SERVERLOGEVENT_FILE_DOWNLOADED             = 0x00200000,
        
        SERVERLOGEVENT_FILE_DELETED                = 0x00400000,
        
        SERVERLOGEVENT_SERVER_UPDATED              = 0x00800000,
        
        SERVERLOGEVENT_SERVER_SAVECONFIG           = 0x01000000,
    } ServerLogEvent;

    
    typedef UINT32 ServerLogEvents;
    
    
    typedef struct ServerProperties
    {
        
        TTCHAR szServerName[TT_STRLEN];
        
        TTCHAR szMOTD[TT_STRLEN];
        
        TTCHAR szMOTDRaw[TT_STRLEN];
        
        INT32 nMaxUsers;
        
        INT32 nMaxLoginAttempts;
        
        INT32 nMaxLoginsPerIPAddress;
        
        INT32 nMaxVoiceTxPerSecond;
        
        INT32 nMaxVideoCaptureTxPerSecond;
        
        INT32 nMaxMediaFileTxPerSecond;
        
        INT32 nMaxDesktopTxPerSecond;
        
        INT32 nMaxTotalTxPerSecond;
        
        TTBOOL bAutoSave;
        
        INT32 nTcpPort;
        
        INT32 nUdpPort;
        
        INT32 nUserTimeout;
        
        TTCHAR szServerVersion[TT_STRLEN];
        
        TTCHAR szServerProtocolVersion[TT_STRLEN];
        
        INT32 nLoginDelayMSec;
        
        TTCHAR szAccessToken[TT_STRLEN];
        
        ServerLogEvents uServerLogEvents;
    } ServerProperties;

    
    typedef struct ServerStatistics
    {
        
        INT64 nTotalBytesTX;
        
        INT64 nTotalBytesRX;
        
        INT64 nVoiceBytesTX;
        
        INT64 nVoiceBytesRX;
        
        INT64 nVideoCaptureBytesTX;
        
        INT64 nVideoCaptureBytesRX;
        
        INT64 nMediaFileBytesTX;
        
        INT64 nMediaFileBytesRX;
        
        INT64 nDesktopBytesTX;
        
        INT64 nDesktopBytesRX;
        
        INT32 nUsersServed;
        
        INT32 nUsersPeak;
        
        INT64 nFilesTx;
        
        INT64 nFilesRx;
        
        INT64 nUptimeMSec;
    } ServerStatistics;

    
    typedef enum BanType
    {
        
        BANTYPE_NONE                = 0x00,
        
        BANTYPE_CHANNEL             = 0x01,
        
        BANTYPE_IPADDR              = 0x02,
        
        BANTYPE_USERNAME            = 0x04
    } BanType;

    
    typedef UINT32 BanTypes;

    
    typedef struct BannedUser
    {
        
        TTCHAR szIPAddress[TT_STRLEN]; 
        
        TTCHAR szChannelPath[TT_STRLEN]; 
        
        TTCHAR szBanTime[TT_STRLEN];
        
        TTCHAR szNickname[TT_STRLEN];
        
        TTCHAR szUsername[TT_STRLEN];
        
        BanTypes uBanTypes;
        
        TTCHAR szOwner[TT_STRLEN];
    } BannedUser;

    
    typedef enum UserType
    {
        
        USERTYPE_NONE    = 0x0, 
        
        USERTYPE_DEFAULT = 0x01, 
        
        USERTYPE_ADMIN   = 0x02 
    } UserType;

    
    typedef UINT32 UserTypes;

    
    typedef struct AbusePrevention
    {
        
        INT32 nCommandsLimit;
        
        INT32 nCommandsIntervalMSec;
    } AbusePrevention;

    
    typedef struct UserAccount
    {
        
        TTCHAR szUsername[TT_STRLEN];
        
        TTCHAR szPassword[TT_STRLEN];
        
        UserTypes uUserType;
        
        UserRights uUserRights;
        
        INT32 nUserData;
        
        TTCHAR szNote[TT_STRLEN];
        
        TTCHAR szInitChannel[TT_STRLEN];
        
        INT32 autoOperatorChannels[TT_CHANNELS_OPERATOR_MAX];
        
        INT32 nAudioCodecBpsLimit;
        
        AbusePrevention abusePrevent;
        
        TTCHAR szLastModified[TT_STRLEN];
        
        TTCHAR szLastLoginTime[TT_STRLEN];
    } UserAccount;
    

    

    
    typedef enum Subscription
    {
        
        SUBSCRIBE_NONE                    = 0x00000000,
        
        SUBSCRIBE_USER_MSG                = 0x00000001,
        
        SUBSCRIBE_CHANNEL_MSG             = 0x00000002,
        
        SUBSCRIBE_BROADCAST_MSG           = 0x00000004,
        
        SUBSCRIBE_CUSTOM_MSG              = 0x00000008,
        
        SUBSCRIBE_VOICE                   = 0x00000010,
        
        SUBSCRIBE_VIDEOCAPTURE            = 0x00000020,
        
        SUBSCRIBE_DESKTOP                 = 0x00000040,
        
        SUBSCRIBE_DESKTOPINPUT            = 0x00000080,
        
        SUBSCRIBE_MEDIAFILE               = 0x00000100,
        
        SUBSCRIBE_INTERCEPT_USER_MSG      = 0x00010000,
        
        SUBSCRIBE_INTERCEPT_CHANNEL_MSG   = 0x00020000,
        
        
        SUBSCRIBE_INTERCEPT_CUSTOM_MSG    = 0x00080000,
        
        SUBSCRIBE_INTERCEPT_VOICE         = 0x00100000,
        
        SUBSCRIBE_INTERCEPT_VIDEOCAPTURE  = 0x00200000,
        
        SUBSCRIBE_INTERCEPT_DESKTOP       = 0x00400000,
        
        
        SUBSCRIBE_INTERCEPT_MEDIAFILE     = 0x01000000,
    } Subscription;

    
    typedef UINT32 Subscriptions;

    
    typedef enum UserState
    {
        
        USERSTATE_NONE                  = 0x0000000,
        
        USERSTATE_VOICE                 = 0x00000001,
        
        USERSTATE_MUTE_VOICE            = 0x00000002,
        
        USERSTATE_MUTE_MEDIAFILE        = 0x00000004,
        
        USERSTATE_DESKTOP               = 0x00000008,
        
        USERSTATE_VIDEOCAPTURE          = 0x00000010,
        
        USERSTATE_MEDIAFILE_AUDIO       = 0x00000020,
        
        USERSTATE_MEDIAFILE_VIDEO       = 0x00000040,
        
        USERSTATE_MEDIAFILE             = USERSTATE_MEDIAFILE_AUDIO |
                                          USERSTATE_MEDIAFILE_VIDEO
    } UserState;

    
    typedef UINT32 UserStates;

    
    typedef struct User
    {
        
        INT32 nUserID;
        
        TTCHAR szUsername[TT_STRLEN];
        
        INT32 nUserData;
        
        UserTypes uUserType;
        
        TTCHAR szIPAddress[TT_STRLEN];
         
        UINT32 uVersion;
        
        INT32 nChannelID; 
        
        Subscriptions uLocalSubscriptions;
        
        Subscriptions uPeerSubscriptions;
         
        TTCHAR szNickname[TT_STRLEN];
        
        INT32 nStatusMode;
        
        TTCHAR szStatusMsg[TT_STRLEN];
        
        UserStates uUserState;
        
        TTCHAR szMediaStorageDir[TT_STRLEN];
        
        INT32 nVolumeVoice;
        
        INT32 nVolumeMediaFile;
        
        INT32 nStoppedDelayVoice;
        
        INT32 nStoppedDelayMediaFile;
        
        float soundPositionVoice[3];
        
        float soundPositionMediaFile[3];
        
        TTBOOL stereoPlaybackVoice[2];
        
        TTBOOL stereoPlaybackMediaFile[2];
        
        INT32 nBufferMSecVoice;
        
        INT32 nBufferMSecMediaFile;
        
        INT32 nActiveAdaptiveDelayMSec;
        
        TTCHAR szClientName[TT_STRLEN];
    } User;

    
    typedef struct UserStatistics
    { 
        
        INT64 nVoicePacketsRecv;
        
        INT64 nVoicePacketsLost;
        
        INT64 nVideoCapturePacketsRecv;
        
        INT64 nVideoCaptureFramesRecv;
        
        INT64 nVideoCaptureFramesLost;
        
        INT64 nVideoCaptureFramesDropped;
        
        INT64 nMediaFileAudioPacketsRecv;
        
        INT64 nMediaFileAudioPacketsLost;
        
        INT64 nMediaFileVideoPacketsRecv;
        
        INT64 nMediaFileVideoFramesRecv;
        
        INT64 nMediaFileVideoFramesLost;
        
        INT64 nMediaFileVideoFramesDropped;
    } UserStatistics;

     
    typedef enum TextMsgType
    {
        
        MSGTYPE_NONE      = 0,
        
        MSGTYPE_USER      = 1,
        
        MSGTYPE_CHANNEL   = 2,
         
        MSGTYPE_BROADCAST = 3,
        
        MSGTYPE_CUSTOM    = 4
    } TextMsgType;

    
    typedef struct TextMessage
    {
        
        TextMsgType nMsgType;
        
        INT32 nFromUserID;
        
        TTCHAR szFromUsername[TT_STRLEN];
        
        INT32 nToUserID;
        
        INT32 nChannelID;
        
        TTCHAR szMessage[TT_STRLEN];
        
        TTBOOL bMore;
    } TextMessage;
    

    

    
    typedef enum ChannelType
    {
        
        CHANNEL_DEFAULT             = 0x0000,
        
        CHANNEL_PERMANENT           = 0x0001,
        
        CHANNEL_SOLO_TRANSMIT       = 0x0002,
        
        CHANNEL_CLASSROOM           = 0x0004,
        
        CHANNEL_OPERATOR_RECVONLY   = 0x0008,
        
        CHANNEL_NO_VOICEACTIVATION  = 0x0010,
        
        CHANNEL_NO_RECORDING        = 0x0020,
        
        CHANNEL_HIDDEN              = 0x0040
    } ChannelType;

    
    typedef UINT32 ChannelTypes;

    
    typedef struct Channel
    {
        
        INT32 nParentID;
        
        INT32 nChannelID;
        
        TTCHAR szName[TT_STRLEN];
        
        TTCHAR szTopic[TT_STRLEN];
        
        TTCHAR szPassword[TT_STRLEN];
        
        TTBOOL bPassword;
        
        ChannelTypes uChannelType;
        
        INT32 nUserData;
        
        INT64 nDiskQuota;
        
        TTCHAR szOpPassword[TT_STRLEN];
        
        INT32 nMaxUsers;
        
        AudioCodec audiocodec;
        
        AudioConfig audiocfg;
        
        INT32 transmitUsers[TT_TRANSMITUSERS_MAX][2];
        
        INT32 transmitUsersQueue[TT_TRANSMITQUEUE_MAX];
        
        INT32 nTransmitUsersQueueDelayMSec;
        
        INT32 nTimeOutTimerVoiceMSec;
        
        INT32 nTimeOutTimerMediaFileMSec;
    } Channel;


    
    typedef enum FileTransferStatus
    {
        
        FILETRANSFER_CLOSED     = 0,
        
        FILETRANSFER_ERROR      = 1,
        
        FILETRANSFER_ACTIVE     = 2,
        
        FILETRANSFER_FINISHED   = 3
    } FileTransferStatus;

    
    typedef struct FileTransfer
    {
        
        FileTransferStatus nStatus;
        
        INT32 nTransferID;
        
        INT32 nChannelID;
        
        TTCHAR szLocalFilePath[TT_STRLEN];
        
        TTCHAR szRemoteFileName[TT_STRLEN];
        
        INT64 nFileSize;
        
        INT64 nTransferred;
        
        TTBOOL bInbound;
    } FileTransfer;


    
    typedef struct RemoteFile
    {
        
        INT32 nChannelID;
        
        INT32 nFileID;
        
        TTCHAR szFileName[TT_STRLEN];
        
        INT64 nFileSize;
        
        TTCHAR szUsername[TT_STRLEN];
        
        TTCHAR szUploadTime[TT_STRLEN];
    } RemoteFile;
    

    
    typedef struct EncryptionContext
    {
        
        TTCHAR szCertificateFile[TT_STRLEN];
        
        TTCHAR szPrivateKeyFile[TT_STRLEN];
        
        TTCHAR szCAFile[TT_STRLEN];
        
        TTCHAR szCADir[TT_STRLEN];
        
        TTBOOL bVerifyPeer;
        
        TTBOOL bVerifyClientOnce;
        
        INT32 nVerifyDepth;
    } EncryptionContext;

    
    typedef struct ClientKeepAlive
    {
        
        INT32 nConnectionLostMSec;
        
        INT32 nTcpKeepAliveIntervalMSec;
        
        INT32 nUdpKeepAliveIntervalMSec;
        
        INT32 nUdpKeepAliveRTXMSec;
        
        INT32 nUdpConnectRTXMSec;
        
        INT32 nUdpConnectTimeoutMSec;
    } ClientKeepAlive;
    
    
    typedef struct ClientStatistics
    {
        
        INT64 nUdpBytesSent;
        
        INT64 nUdpBytesRecv;
        
        INT64 nVoiceBytesSent;
        
        INT64 nVoiceBytesRecv;
        
        INT64 nVideoCaptureBytesSent;
        
        INT64 nVideoCaptureBytesRecv;
        
        INT64 nMediaFileAudioBytesSent;
        
        INT64 nMediaFileAudioBytesRecv;
        
        INT64 nMediaFileVideoBytesSent;
        
        INT64 nMediaFileVideoBytesRecv;
        
        INT64 nDesktopBytesSent;
        
        INT64 nDesktopBytesRecv;
        
        INT32 nUdpPingTimeMs;
        
        INT32 nTcpPingTimeMs;
        
        INT32 nTcpServerSilenceSec;
        
        INT32 nUdpServerSilenceSec;
        
        INT32 nSoundInputDeviceDelayMSec;
    } ClientStatistics;

    
    typedef struct JitterConfig
    {
        
        INT32 nFixedDelayMSec;
        
        TTBOOL bUseAdativeDejitter;
        
        INT32 nMaxAdaptiveDelayMSec;
        
        INT32 nActiveAdaptiveDelayMSec;
    } JitterConfig;

    

    
    typedef enum ClientError
    {
        
        CMDERR_SUCCESS = 0,

        

        
        CMDERR_SYNTAX_ERROR = 1000,
        
        CMDERR_UNKNOWN_COMMAND = 1001,
        
        CMDERR_MISSING_PARAMETER = 1002,
        
        CMDERR_INCOMPATIBLE_PROTOCOLS = 1003,
        
        CMDERR_UNKNOWN_AUDIOCODEC = 1004,
        
        CMDERR_INVALID_USERNAME = 1005,

        

        
        CMDERR_INCORRECT_CHANNEL_PASSWORD = 2001,
        
        CMDERR_INVALID_ACCOUNT = 2002,
        
        CMDERR_MAX_SERVER_USERS_EXCEEDED = 2003,
        
        CMDERR_MAX_CHANNEL_USERS_EXCEEDED = 2004,
        
        CMDERR_SERVER_BANNED = 2005,
        
        CMDERR_NOT_AUTHORIZED = 2006,
        
        CMDERR_MAX_DISKUSAGE_EXCEEDED = 2008,
        
        CMDERR_INCORRECT_OP_PASSWORD = 2010,

        
        CMDERR_AUDIOCODEC_BITRATE_LIMIT_EXCEEDED = 2011,

        
        CMDERR_MAX_LOGINS_PER_IPADDRESS_EXCEEDED = 2012,
        
        
        CMDERR_MAX_CHANNELS_EXCEEDED = 2013,

        
        CMDERR_COMMAND_FLOOD = 2014,

        
        CMDERR_CHANNEL_BANNED = 2015,

        
        CMDERR_MAX_FILETRANSFERS_EXCEEDED = 2016,

        

        
        CMDERR_NOT_LOGGEDIN = 3000,

        
        CMDERR_ALREADY_LOGGEDIN = 3001,
        
        CMDERR_NOT_IN_CHANNEL = 3002,
        
        CMDERR_ALREADY_IN_CHANNEL = 3003,
        
        CMDERR_CHANNEL_ALREADY_EXISTS = 3004,
        
        CMDERR_CHANNEL_NOT_FOUND = 3005,
        
        CMDERR_USER_NOT_FOUND = 3006,
        
        CMDERR_BAN_NOT_FOUND = 3007,
        
        CMDERR_FILETRANSFER_NOT_FOUND = 3008,
        
        CMDERR_OPENFILE_FAILED = 3009,
        
        CMDERR_ACCOUNT_NOT_FOUND = 3010,
        
        CMDERR_FILE_NOT_FOUND = 3011,
        
        CMDERR_FILE_ALREADY_EXISTS = 3012,
        
        CMDERR_FILESHARING_DISABLED = 3013,
        
        CMDERR_CHANNEL_HAS_USERS = 3015,

        
        CMDERR_LOGINSERVICE_UNAVAILABLE = 3016,

        
        CMDERR_CHANNEL_CANNOT_BE_HIDDEN = 3017,

        

        
        INTERR_SNDINPUT_FAILURE = 10000,
        
        INTERR_SNDOUTPUT_FAILURE = 10001,
        
        INTERR_AUDIOCODEC_INIT_FAILED = 10002,
        
        INTERR_SPEEXDSP_INIT_FAILED = 10003,
        
        INTERR_AUDIOPREPROCESSOR_INIT_FAILED = 10003,
        
        INTERR_TTMESSAGE_QUEUE_OVERFLOW = 10004,
        
        INTERR_SNDEFFECT_FAILURE = 10005,
    } ClientError;

    
    typedef struct ClientErrorMsg
    {
        
        INT32 nErrorNo;
        
        TTCHAR szErrorMsg[TT_STRLEN];
    } ClientErrorMsg;
    

    

    
    typedef enum ClientEvent
    {
        CLIENTEVENT_NONE = 0,

        
        CLIENTEVENT_CON_SUCCESS = CLIENTEVENT_NONE + 10,
        
        CLIENTEVENT_CON_CRYPT_ERROR = CLIENTEVENT_NONE + 15,
        
        CLIENTEVENT_CON_FAILED = CLIENTEVENT_NONE + 20,
        
        CLIENTEVENT_CON_LOST = CLIENTEVENT_NONE + 30,
        
        CLIENTEVENT_CON_MAX_PAYLOAD_UPDATED = CLIENTEVENT_NONE + 40,
        
        CLIENTEVENT_CMD_PROCESSING = CLIENTEVENT_NONE + 200,
        
        CLIENTEVENT_CMD_ERROR = CLIENTEVENT_NONE + 210,
        
        CLIENTEVENT_CMD_SUCCESS = CLIENTEVENT_NONE + 220,
        
        CLIENTEVENT_CMD_MYSELF_LOGGEDIN = CLIENTEVENT_NONE + 230,
        
        CLIENTEVENT_CMD_MYSELF_LOGGEDOUT = CLIENTEVENT_NONE + 240,
        
        CLIENTEVENT_CMD_MYSELF_KICKED = CLIENTEVENT_NONE + 250,
        
        CLIENTEVENT_CMD_USER_LOGGEDIN = CLIENTEVENT_NONE + 260,
        
        CLIENTEVENT_CMD_USER_LOGGEDOUT = CLIENTEVENT_NONE + 270,
        
        CLIENTEVENT_CMD_USER_UPDATE = CLIENTEVENT_NONE + 280,
        
        CLIENTEVENT_CMD_USER_JOINED = CLIENTEVENT_NONE + 290,
        
        CLIENTEVENT_CMD_USER_LEFT = CLIENTEVENT_NONE + 300,
        
        CLIENTEVENT_CMD_USER_TEXTMSG = CLIENTEVENT_NONE + 310,
        
        CLIENTEVENT_CMD_CHANNEL_NEW = CLIENTEVENT_NONE + 320,
        
        CLIENTEVENT_CMD_CHANNEL_UPDATE = CLIENTEVENT_NONE + 330,
        
        CLIENTEVENT_CMD_CHANNEL_REMOVE = CLIENTEVENT_NONE + 340,
        
        CLIENTEVENT_CMD_SERVER_UPDATE = CLIENTEVENT_NONE + 350,
        
        CLIENTEVENT_CMD_SERVERSTATISTICS = CLIENTEVENT_NONE + 360,
        
        CLIENTEVENT_CMD_FILE_NEW = CLIENTEVENT_NONE + 370,
        
        CLIENTEVENT_CMD_FILE_REMOVE = CLIENTEVENT_NONE + 380,
        
        CLIENTEVENT_CMD_USERACCOUNT = CLIENTEVENT_NONE + 390,
        
        CLIENTEVENT_CMD_BANNEDUSER  = CLIENTEVENT_NONE + 400,
        
        CLIENTEVENT_CMD_USERACCOUNT_NEW = CLIENTEVENT_NONE + 410,
        
        CLIENTEVENT_CMD_USERACCOUNT_REMOVE = CLIENTEVENT_NONE + 420,
        
        CLIENTEVENT_USER_STATECHANGE = CLIENTEVENT_NONE + 500,
        
        CLIENTEVENT_USER_VIDEOCAPTURE = CLIENTEVENT_NONE + 510,
        
        CLIENTEVENT_USER_MEDIAFILE_VIDEO = CLIENTEVENT_NONE + 520,
        
        CLIENTEVENT_USER_DESKTOPWINDOW = CLIENTEVENT_NONE + 530,
        
        CLIENTEVENT_USER_DESKTOPCURSOR = CLIENTEVENT_NONE + 540,
        
        CLIENTEVENT_USER_DESKTOPINPUT = CLIENTEVENT_NONE + 550,
        
        CLIENTEVENT_USER_RECORD_MEDIAFILE = CLIENTEVENT_NONE + 560,
        
        CLIENTEVENT_USER_AUDIOBLOCK = CLIENTEVENT_NONE + 570,
        
        CLIENTEVENT_INTERNAL_ERROR = CLIENTEVENT_NONE + 1000,
        
        CLIENTEVENT_VOICE_ACTIVATION = CLIENTEVENT_NONE + 1010,
        
        CLIENTEVENT_HOTKEY = CLIENTEVENT_NONE + 1020,
        
        CLIENTEVENT_HOTKEY_TEST = CLIENTEVENT_NONE + 1030,
        
        CLIENTEVENT_FILETRANSFER = CLIENTEVENT_NONE + 1040,
        
        CLIENTEVENT_DESKTOPWINDOW_TRANSFER = CLIENTEVENT_NONE + 1050,
        
        CLIENTEVENT_STREAM_MEDIAFILE = CLIENTEVENT_NONE + 1060,
        
         CLIENTEVENT_LOCAL_MEDIAFILE = CLIENTEVENT_NONE + 1070,
        
        CLIENTEVENT_AUDIOINPUT = CLIENTEVENT_NONE + 1080,
        
        CLIENTEVENT_USER_FIRSTVOICESTREAMPACKET = CLIENTEVENT_NONE + 1090,
        
        CLIENTEVENT_SOUNDDEVICE_ADDED = CLIENTEVENT_NONE + 1100,
        
        CLIENTEVENT_SOUNDDEVICE_REMOVED = CLIENTEVENT_NONE + 1110,
        
        CLIENTEVENT_SOUNDDEVICE_UNPLUGGED = CLIENTEVENT_NONE + 1120,
        
        CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_INPUT = CLIENTEVENT_NONE + 1130,
        
        CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_OUTPUT = CLIENTEVENT_NONE + 1140,
        
        CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_INPUT_COMDEVICE = CLIENTEVENT_NONE + 1150,
        
        CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_OUTPUT_COMDEVICE = CLIENTEVENT_NONE + 1160,
        
    } ClientEvent;

    
    typedef enum TTType
    {
        __NONE                    =  0,
        __AUDIOCODEC              =  1,
        __BANNEDUSER              =  2,
        __VIDEOFORMAT             =  3,
        __OPUSCODEC               =  4,
        __CHANNEL                 =  5,
        __CLIENTSTATISTICS        =  6,
        __REMOTEFILE              =  7,
        __FILETRANSFER            =  8,
        __MEDIAFILESTATUS         =  9,
        __SERVERPROPERTIES        = 10,
        __SERVERSTATISTICS        = 11,
        __SOUNDDEVICE             = 12,
        __SPEEXCODEC              = 13,
        __TEXTMESSAGE             = 14,
        __WEBMVP8CODEC            = 15,
        __TTMESSAGE               = 16,
        __USER                    = 17,
        __USERACCOUNT             = 18,
        __USERSTATISTICS          = 19,
        __VIDEOCAPTUREDEVICE      = 20,
        __VIDEOCODEC              = 21,
        __AUDIOCONFIG             = 22,
        __SPEEXVBRCODEC           = 23,
        __VIDEOFRAME              = 24,
        __AUDIOBLOCK              = 25,
        __AUDIOFORMAT             = 26,
        __MEDIAFILEINFO           = 27,
        __CLIENTERRORMSG          = 28,
        __TTBOOL                  = 29,
        __INT32                   = 30,
        __DESKTOPINPUT            = 31,
        __SPEEXDSP                = 32,
        __STREAMTYPE              = 33,
        __AUDIOPREPROCESSORTYPE   = 34,
        __AUDIOPREPROCESSOR       = 35,
        __TTAUDIOPREPROCESSOR     = 36,
        __MEDIAFILEPLAYBACK       = 37,
        __CLIENTKEEPALIVE         = 38,
        __UINT32                  = 39,
        __AUDIOINPUTPROGRESS      = 40,
        __JITTERCONFIG            = 41,
        __WEBRTCAUDIOPREPROCESSOR = 42,
        __ENCRYPTIONCONTEXT       = 43,
        __SOUNDDEVICEEFFECTS       = 44,
        __DESKTOPWINDOW       = 45,
        __ABUSEPREVENTION       = 46,
    } TTType;

    
    typedef struct TTMessage
    {
        
        ClientEvent nClientEvent;
        
        INT32 nSource;
        
        TTType ttType;
        
        UINT32 uReserved;
        union
        {
            
            Channel channel;
            
            ClientErrorMsg clienterrormsg;
            
            DesktopInput desktopinput;
            
            FileTransfer filetransfer;
            
            MediaFileInfo mediafileinfo;
            
            RemoteFile remotefile;
            
            ServerProperties serverproperties;
            
            ServerStatistics serverstatistics;
            
            TextMessage textmessage;
            
            User user;
            
            UserAccount useraccount;
            
            BannedUser banneduser;
            
            TTBOOL bActive;
            
            INT32 nBytesRemain;
            
            INT32 nStreamID;
            
            INT32 nPayloadSize;
            
            StreamType nStreamType;
            
            AudioInputProgress audioinputprogress;
            
            SoundDevice sounddevice;
            
            char data[1];
        };
    } TTMessage;

    


    

    
    typedef enum ClientFlag
    {
        
        CLIENT_CLOSED                   = 0x00000000,
        
        CLIENT_SNDINPUT_READY           = 0x00000001,
        
        CLIENT_SNDOUTPUT_READY          = 0x00000002,
        
        CLIENT_SNDINOUTPUT_DUPLEX       = 0x00000004,
        
        CLIENT_SNDINPUT_VOICEACTIVATED  = 0x00000008,
        
        CLIENT_SNDINPUT_VOICEACTIVE     = 0x00000010,
        
        CLIENT_SNDOUTPUT_MUTE           = 0x00000020,
        
        CLIENT_SNDOUTPUT_AUTO3DPOSITION = 0x00000040,
        
        CLIENT_VIDEOCAPTURE_READY       = 0x00000080,
        
        CLIENT_TX_VOICE                 = 0x00000100,
        
        CLIENT_TX_VIDEOCAPTURE          = 0x00000200,
        
        CLIENT_TX_DESKTOP               = 0x00000400,
        
        CLIENT_DESKTOP_ACTIVE           = 0x00000800,
        
        CLIENT_MUX_AUDIOFILE            = 0x00001000,
        
        CLIENT_CONNECTING               = 0x00002000,
        
        CLIENT_CONNECTED                = 0x00004000,
        
        CLIENT_CONNECTION               = CLIENT_CONNECTING | CLIENT_CONNECTED,
        
        CLIENT_AUTHORIZED               = 0x00008000,
        
        CLIENT_STREAM_AUDIO             = 0x00010000,
        
        CLIENT_STREAM_VIDEO             = 0x00020000
    } ClientFlag;

    
    typedef UINT32 ClientFlags;
    
    
    typedef VOID TTInstance;

    
    typedef VOID TTSoundLoop;

    
    TEAMTALKDLL_API const TTCHAR* TT_GetVersion(void);

#if defined(WIN32)
    
    TEAMTALKDLL_API TTInstance* TT_InitTeamTalk(IN HWND hWnd, IN UINT32 uMsg);

    
    TEAMTALKDLL_API TTBOOL TT_SwapTeamTalkHWND(IN TTInstance* lpTTInstance,
                                               IN HWND hWnd);
#endif

    
    TEAMTALKDLL_API TTInstance* TT_InitTeamTalkPoll(void);

    
    TEAMTALKDLL_API TTBOOL TT_CloseTeamTalk(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_GetMessage(IN TTInstance* lpTTInstance, 
                                         OUT TTMessage* pMsg,
                                         IN const INT32* pnWaitMs);

    
    TEAMTALKDLL_API TTBOOL TT_PumpMessage(IN TTInstance* lpTTInstance,
                                          ClientEvent nClientEvent,
                                          INT32 nIdentifier);

    
     TEAMTALKDLL_API ClientFlags TT_GetFlags(IN TTInstance* lpTTInstance);

     
     TEAMTALKDLL_API TTBOOL TT_SetLicenseInformation(IN const TTCHAR szRegName[TT_STRLEN],
                                                     IN const TTCHAR szRegKey[TT_STRLEN]);
    

    

    
    TEAMTALKDLL_API TTBOOL TT_GetDefaultSoundDevices(OUT INT32* lpnInputDeviceID, 
                                                     OUT INT32* lpnOutputDeviceID);
    
    TEAMTALKDLL_API TTBOOL TT_GetDefaultSoundDevicesEx(IN SoundSystem nSndSystem, 
                                                       OUT INT32* lpnInputDeviceID, 
                                                       OUT INT32* lpnOutputDeviceID);

    
    TEAMTALKDLL_API TTBOOL TT_GetSoundDevices(IN OUT SoundDevice* lpSoundDevices,
                                              IN OUT INT32* lpnHowMany);


    
    TEAMTALKDLL_API TTBOOL TT_RestartSoundSystem(void);

    
    TEAMTALKDLL_API TTSoundLoop* TT_StartSoundLoopbackTest(IN INT32 nInputDeviceID,
                                                           IN INT32 nOutputDeviceID,
                                                           IN INT32 nSampleRate,
                                                           IN INT32 nChannels,
                                                           IN TTBOOL bDuplexMode,
                                                           IN const SpeexDSP* lpSpeexDSP);

    
    TEAMTALKDLL_API TTSoundLoop* TT_StartSoundLoopbackTestEx(IN INT32 nInputDeviceID,
                                                             IN INT32 nOutputDeviceID,
                                                             IN INT32 nSampleRate,
                                                             IN INT32 nChannels,
                                                             IN TTBOOL bDuplexMode,
                                                             IN const AudioPreprocessor* lpAudioPreprocessor,
                                                             IN const SoundDeviceEffects* lpSoundDeviceEffects);
    
    
    
    TEAMTALKDLL_API TTBOOL TT_CloseSoundLoopbackTest(IN TTSoundLoop* lpTTSoundLoop);

    
    TEAMTALKDLL_API TTBOOL TT_InitSoundInputDevice(IN TTInstance* lpTTInstance, 
                                                   IN INT32 nInputDeviceID);

    
    TEAMTALKDLL_API TTBOOL TT_InitSoundInputSharedDevice(IN INT32 nSampleRate,
                                                         IN INT32 nChannels,
                                                         IN INT32 nFrameSize);
    
    TEAMTALKDLL_API TTBOOL TT_InitSoundOutputDevice(IN TTInstance* lpTTInstance, 
                                                    IN INT32 nOutputDeviceID);

    
    TEAMTALKDLL_API TTBOOL TT_InitSoundOutputSharedDevice(IN INT32 nSampleRate,
                                                          IN INT32 nChannels,
                                                          IN INT32 nFrameSize);
    
    TEAMTALKDLL_API TTBOOL TT_InitSoundDuplexDevices(IN TTInstance* lpTTInstance, 
                                                     IN INT32 nInputDeviceID,
                                                     IN INT32 nOutputDeviceID);

    
    TEAMTALKDLL_API TTBOOL TT_CloseSoundInputDevice(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_CloseSoundOutputDevice(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_CloseSoundDuplexDevices(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_SetSoundDeviceEffects(IN TTInstance* lpTTInstance,
                                                    IN const SoundDeviceEffects* lpSoundDeviceEffect);

    
    TEAMTALKDLL_API TTBOOL TT_GetSoundDeviceEffects(IN TTInstance* lpTTInstance,
                                                    OUT SoundDeviceEffects* lpSoundDeviceEffect);
    
    
    TEAMTALKDLL_API INT32 TT_GetSoundInputLevel(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_SetSoundInputGainLevel(IN TTInstance* lpTTInstance, 
                                                     IN INT32 nLevel);

    
    TEAMTALKDLL_API INT32 TT_GetSoundInputGainLevel(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_SetSoundInputPreprocess(IN TTInstance* lpTTInstance,
                                                      IN const SpeexDSP* lpSpeexDSP);

    
    TEAMTALKDLL_API TTBOOL TT_GetSoundInputPreprocess(IN TTInstance* lpTTInstance,
                                                      OUT SpeexDSP* lpSpeexDSP);

    
    TEAMTALKDLL_API TTBOOL TT_SetSoundInputPreprocessEx(IN TTInstance* lpTTInstance,
                                                        IN const AudioPreprocessor* lpAudioPreprocessor);
    
    
    TEAMTALKDLL_API TTBOOL TT_GetSoundInputPreprocessEx(IN TTInstance* lpTTInstance,
                                                        OUT AudioPreprocessor* lpAudioPreprocessor);

    
    TEAMTALKDLL_API TTBOOL TT_SetSoundOutputVolume(IN TTInstance* lpTTInstance, 
                                                   IN INT32 nVolume);

    
    TEAMTALKDLL_API INT32 TT_GetSoundOutputVolume(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_SetSoundOutputMute(IN TTInstance* lpTTInstance, 
                                                 IN TTBOOL bMuteAll);

    
    TEAMTALKDLL_API TTBOOL TT_Enable3DSoundPositioning(IN TTInstance* lpTTInstance, 
                                                       IN TTBOOL bEnable);

    
    TEAMTALKDLL_API TTBOOL TT_AutoPositionUsers(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_EnableAudioBlockEvent(IN TTInstance* lpTTInstance,
                                                    IN INT32 nUserID,
                                                    IN StreamTypes uStreamTypes,
                                                    IN TTBOOL bEnable);

    
    TEAMTALKDLL_API TTBOOL TT_EnableAudioBlockEventEx(IN TTInstance* lpTTInstance,
                                                      IN INT32 nUserID,
                                                      IN StreamTypes uStreamTypes,
                                                      IN const AudioFormat* lpAudioFormat,
                                                      IN TTBOOL bEnable);
    
    

    

    
    TEAMTALKDLL_API TTBOOL TT_InsertAudioBlock(IN TTInstance* lpTTInstance,
                                               IN const AudioBlock* lpAudioBlock);
    
    
    TEAMTALKDLL_API TTBOOL TT_EnableVoiceTransmission(IN TTInstance* lpTTInstance,
                                                      IN TTBOOL bEnable);

    
    TEAMTALKDLL_API TTBOOL TT_EnableVoiceActivation(IN TTInstance* lpTTInstance, 
                                                    IN TTBOOL bEnable);

    
    TEAMTALKDLL_API TTBOOL TT_SetVoiceActivationLevel(IN TTInstance* lpTTInstance, 
                                                      IN INT32 nLevel);

    
    TEAMTALKDLL_API INT32 TT_GetVoiceActivationLevel(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_SetVoiceActivationStopDelay(IN TTInstance* lpTTInstance,
                                                          IN INT32 nDelayMSec);

    
    TEAMTALKDLL_API INT32 TT_GetVoiceActivationStopDelay(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_StartRecordingMuxedAudioFile(IN TTInstance* lpTTInstance,
                                                           IN const AudioCodec* lpAudioCodec,
                                                           IN const TTCHAR* szAudioFileName,
                                                           IN AudioFileFormat uAFF);

    
    TEAMTALKDLL_API TTBOOL TT_StartRecordingMuxedAudioFileEx(IN TTInstance* lpTTInstance,
                                                             IN INT32 nChannelID,
                                                             IN const TTCHAR* szAudioFileName,
                                                             IN AudioFileFormat uAFF);

    
    TEAMTALKDLL_API TTBOOL TT_StartRecordingMuxedStreams(IN TTInstance* lpTTInstance,
                                                         IN StreamTypes uStreamTypes,
                                                         IN const AudioCodec* lpAudioCodec,
                                                         IN const TTCHAR* szAudioFileName,
                                                         IN AudioFileFormat uAFF);

    
    TEAMTALKDLL_API TTBOOL TT_StopRecordingMuxedAudioFile(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_StopRecordingMuxedAudioFileEx(IN TTInstance* lpTTInstance,
                                                            IN INT32 nChannelID);
    
    
    TEAMTALKDLL_API TTBOOL TT_StartVideoCaptureTransmission(IN TTInstance* lpTTInstance,
                                                            IN const VideoCodec* lpVideoCodec);

    
    TEAMTALKDLL_API TTBOOL TT_StopVideoCaptureTransmission(IN TTInstance* lpTTInstance);

    

    

    
    TEAMTALKDLL_API TTBOOL TT_GetVideoCaptureDevices(IN OUT VideoCaptureDevice* lpVideoDevices,
                                                     IN OUT INT32* lpnHowMany);

    
    TEAMTALKDLL_API TTBOOL TT_InitVideoCaptureDevice(IN TTInstance* lpTTInstance,
                                                     IN const TTCHAR* szDeviceID,
                                                     IN const VideoFormat* lpVideoFormat);
    
    TEAMTALKDLL_API TTBOOL TT_CloseVideoCaptureDevice(IN TTInstance* lpTTInstance);

#if defined(WIN32)
    
    TEAMTALKDLL_API TTBOOL TT_PaintVideoFrame(IN HDC hDC,
                                              IN INT32 XDest,
                                              IN INT32 YDest,
                                              IN INT32 nDestWidth,
                                              IN INT32 nDestHeight,
                                              IN VideoFrame* lpVideoFrame);

    
    TEAMTALKDLL_API TTBOOL TT_PaintVideoFrameEx(IN HDC hDC,
                                                IN INT32 XDest,
                                                IN INT32 YDest,
                                                IN INT32 nDestWidth,
                                                IN INT32 nDestHeight,
                                                IN INT32 XSrc,
                                                IN INT32 YSrc,
                                                IN INT32 nSrcWidth,
                                                IN INT32 nSrcHeight,
                                                IN VideoFrame* lpVideoFrame);
#endif

    
    TEAMTALKDLL_API VideoFrame* TT_AcquireUserVideoCaptureFrame(IN TTInstance* lpTTInstance,
                                                                IN INT32 nUserID);

    
    TEAMTALKDLL_API TTBOOL TT_ReleaseUserVideoCaptureFrame(IN TTInstance* lpTTInstance,
                                                           IN VideoFrame* lpVideoFrame);
    

    

    
    TEAMTALKDLL_API TTBOOL TT_StartStreamingMediaFileToChannel(IN TTInstance* lpTTInstance,
                                                               IN const TTCHAR* szMediaFilePath,
                                                               IN const VideoCodec* lpVideoCodec);

    
    TEAMTALKDLL_API TTBOOL TT_StartStreamingMediaFileToChannelEx(IN TTInstance* lpTTInstance,
                                                                 IN const TTCHAR* szMediaFilePath,
                                                                 IN const MediaFilePlayback* lpMediaFilePlayback,
                                                                 IN const VideoCodec* lpVideoCodec);

    
    TEAMTALKDLL_API TTBOOL TT_UpdateStreamingMediaFileToChannel(IN TTInstance* lpTTInstance,
                                                                IN const MediaFilePlayback* lpMediaFilePlayback,
                                                                IN const VideoCodec* lpVideoCodec);

    
    TEAMTALKDLL_API TTBOOL TT_StopStreamingMediaFileToChannel(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_InitLocalPlayback(IN TTInstance* lpTTInstance,
                                               IN const TTCHAR* szMediaFilePath,
                                               IN const MediaFilePlayback* lpMediaFilePlayback);

    
    TEAMTALKDLL_API TTBOOL TT_UpdateLocalPlayback(IN TTInstance* lpTTInstance,
                                                  IN INT32 nPlaybackSessionID,
                                                  IN const MediaFilePlayback* lpMediaFilePlayback);

    
    TEAMTALKDLL_API TTBOOL TT_StopLocalPlayback(IN TTInstance* lpTTInstance,
                                                IN INT32 nPlaybackSessionID);
    
    
    TEAMTALKDLL_API TTBOOL TT_GetMediaFileInfo(IN const TTCHAR* szMediaFilePath,
                                               OUT MediaFileInfo* lpMediaFileInfo);

    
    TEAMTALKDLL_API VideoFrame* TT_AcquireUserMediaVideoFrame(IN TTInstance* lpTTInstance,
                                                              IN INT32 nUserID);

    
    TEAMTALKDLL_API TTBOOL TT_ReleaseUserMediaVideoFrame(IN TTInstance* lpTTInstance,
                                                         IN VideoFrame* lpVideoFrame);
    

    

    
    TEAMTALKDLL_API INT32 TT_SendDesktopWindow(IN TTInstance* lpTTInstance,
                                               IN const DesktopWindow* lpDesktopWindow,
                                               IN BitmapFormat nConvertBmpFormat);

    
    TEAMTALKDLL_API TTBOOL TT_CloseDesktopWindow(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API unsigned char* TT_Palette_GetColorTable(IN BitmapFormat nBmpPalette,
                                                            IN INT32 nIndex);
#if defined(WIN32)

    
    TEAMTALKDLL_API HWND TT_Windows_GetDesktopActiveHWND(void);

    
    TEAMTALKDLL_API HWND TT_Windows_GetDesktopHWND(void);

    
    TEAMTALKDLL_API TTBOOL TT_Windows_GetDesktopWindowHWND(IN INT32 nIndex,
                                                           OUT HWND* lpHWnd);

    
    typedef struct ShareWindow
    {
        
        HWND hWnd;
        
        INT32 nWndX;
        
        INT32 nWndY;
        
        INT32 nWidth;
        
        INT32 nHeight;
        
        TTCHAR szWindowTitle[TT_STRLEN];
    } ShareWindow;

    
    TEAMTALKDLL_API TTBOOL TT_Windows_GetWindow(IN HWND hWnd,
                                                OUT ShareWindow* lpShareWindow);

    
    TEAMTALKDLL_API INT32 TT_SendDesktopWindowFromHWND(IN TTInstance* lpTTInstance,
                                                       IN HWND hWnd, 
                                                       IN BitmapFormat nBitmapFormat,
                                                       IN DesktopProtocol nDesktopProtocol);
    
    
    TEAMTALKDLL_API TTBOOL TT_PaintDesktopWindow(IN TTInstance* lpTTInstance,
                                                 IN INT32 nUserID,
                                                 IN HDC hDC,
                                                 IN INT32 XDest,
                                                 IN INT32 YDest,
                                                 IN INT32 nDestWidth,
                                                 IN INT32 nDestHeight);

    
    TEAMTALKDLL_API TTBOOL TT_PaintDesktopWindowEx(IN TTInstance* lpTTInstance,
                                                   IN INT32 nUserID,
                                                   IN HDC hDC,
                                                   IN INT32 XDest,
                                                   IN INT32 YDest,
                                                   IN INT32 nDestWidth,
                                                   IN INT32 nDestHeight,
                                                   IN INT32 XSrc,
                                                   IN INT32 YSrc,
                                                   IN INT32 nSrcWidth,
                                                   IN INT32 nSrcHeight);
#endif

#if defined(__APPLE__)

    
    typedef struct ShareWindow
    {
        
        INT64 nWindowID;
        
        INT32 nWindowX;
        
        INT32 nWindowY;
        
        INT32 nWidth;
        
        INT32 nHeight;
        
        TTCHAR szWindowTitle[TT_STRLEN];
        
        INT64 nPID;
    } ShareWindow;

    
    TEAMTALKDLL_API TTBOOL TT_MacOS_GetWindow(IN INT32 nIndex,
                                              OUT ShareWindow* lpShareWindow);

    
    TEAMTALKDLL_API TTBOOL TT_MacOS_GetWindowFromWindowID(IN INT64 nWindowID,
                                                          OUT ShareWindow* lpShareWindow);

    
    TEAMTALKDLL_API INT32 TT_SendDesktopFromWindowID(IN TTInstance* lpTTInstance,
                                                     IN INT64 nWindowID, 
                                                     IN BitmapFormat nBitmapFormat,
                                                     IN DesktopProtocol nDesktopProtocol);
#endif

    
    TEAMTALKDLL_API TTBOOL TT_SendDesktopCursorPosition(IN TTInstance* lpTTInstance,
                                                        IN UINT16 nPosX,
                                                        IN UINT16 nPosY);
    
    TEAMTALKDLL_API TTBOOL TT_SendDesktopInput(IN TTInstance* lpTTInstance,
                                               IN INT32 nUserID,
                                               IN const DesktopInput lpDesktopInputs[TT_DESKTOPINPUT_MAX],
                                               IN INT32 nDesktopInputCount);

    
    TEAMTALKDLL_API DesktopWindow* TT_AcquireUserDesktopWindow(IN TTInstance* lpTTInstance, 
                                                               IN INT32 nUserID);

    
    TEAMTALKDLL_API DesktopWindow* TT_AcquireUserDesktopWindowEx(IN TTInstance* lpTTInstance, 
                                                                 IN INT32 nUserID,
                                                                 IN BitmapFormat nBitmapFormat);

    
    TEAMTALKDLL_API TTBOOL TT_ReleaseUserDesktopWindow(IN TTInstance* lpTTInstance, 
                                                       IN DesktopWindow* lpDesktopWindow);
    

    

    
    TEAMTALKDLL_API TTBOOL TT_SetEncryptionContext(IN TTInstance* lpTTInstance,
                                                   const EncryptionContext* lpEncryptionContext);

    
    TEAMTALKDLL_API TTBOOL TT_Connect(IN TTInstance* lpTTInstance,
                                      IN const TTCHAR* szHostAddress, 
                                      IN INT32 nTcpPort, 
                                      IN INT32 nUdpPort, 
                                      IN INT32 nLocalTcpPort, 
                                      IN INT32 nLocalUdpPort,
                                      IN TTBOOL bEncrypted);

    
    TEAMTALKDLL_API TTBOOL TT_ConnectSysID(IN TTInstance* lpTTInstance,
                                           IN const TTCHAR* szHostAddress, 
                                           IN INT32 nTcpPort, 
                                           IN INT32 nUdpPort, 
                                           IN INT32 nLocalTcpPort, 
                                           IN INT32 nLocalUdpPort,
                                           IN TTBOOL bEncrypted,
                                           IN const TTCHAR* szSystemID);

    
    TEAMTALKDLL_API TTBOOL TT_ConnectEx(IN TTInstance* lpTTInstance,
                                        IN const TTCHAR* szHostAddress,
                                        IN INT32 nTcpPort,
                                        IN INT32 nUdpPort,
                                        IN const TTCHAR* szBindIPAddr,
                                        IN INT32 nLocalTcpPort,
                                        IN INT32 nLocalUdpPort,
                                        IN TTBOOL bEncrypted);

    
    TEAMTALKDLL_API TTBOOL TT_Disconnect(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_QueryMaxPayload(IN TTInstance* lpTTInstance,
                                              IN INT32 nUserID);
    
    
     TEAMTALKDLL_API TTBOOL TT_GetClientStatistics(IN TTInstance* lpTTInstance,
                                                   OUT ClientStatistics* lpClientStatistics);

    
    TEAMTALKDLL_API TTBOOL TT_SetClientKeepAlive(IN TTInstance* lpTTInstance,
                                                 IN const ClientKeepAlive* lpClientKeepAlive);

    
    TEAMTALKDLL_API TTBOOL TT_GetClientKeepAlive(IN TTInstance* lpTTInstance,
                                                 OUT ClientKeepAlive* lpClientKeepAlive);
    
    

    

    
    TEAMTALKDLL_API INT32 TT_DoPing(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_DoLogin(IN TTInstance* lpTTInstance,
                                     IN const TTCHAR* szNickname, 
                                     IN const TTCHAR* szUsername,
                                     IN const TTCHAR* szPassword);


    
    TEAMTALKDLL_API INT32 TT_DoLoginEx(IN TTInstance* lpTTInstance,
                                       IN const TTCHAR* szNickname, 
                                       IN const TTCHAR* szUsername,
                                       IN const TTCHAR* szPassword,
                                       IN const TTCHAR* szClientName);

    
    TEAMTALKDLL_API INT32 TT_DoLogout(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_DoJoinChannel(IN TTInstance* lpTTInstance,
                                           IN const Channel* lpChannel);

    
    TEAMTALKDLL_API INT32 TT_DoJoinChannelByID(IN TTInstance* lpTTInstance,
                                               IN INT32 nChannelID, 
                                               IN const TTCHAR* szPassword);

    
    TEAMTALKDLL_API INT32 TT_DoLeaveChannel(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_DoChangeNickname(IN TTInstance* lpTTInstance, 
                                              IN const TTCHAR* szNewNick);

    
    TEAMTALKDLL_API INT32 TT_DoChangeStatus(IN TTInstance* lpTTInstance,
                                            IN INT32 nStatusMode, 
                                            IN const TTCHAR* szStatusMessage);

    
    TEAMTALKDLL_API INT32 TT_DoTextMessage(IN TTInstance* lpTTInstance,
                                           IN const TextMessage* lpTextMessage);

    
    TEAMTALKDLL_API INT32 TT_DoChannelOp(IN TTInstance* lpTTInstance,
                                         IN INT32 nUserID,
                                         IN INT32 nChannelID,
                                         IN TTBOOL bMakeOperator);

    
    TEAMTALKDLL_API INT32 TT_DoChannelOpEx(IN TTInstance* lpTTInstance,
                                           IN INT32 nUserID,
                                           IN INT32 nChannelID,
                                           IN const TTCHAR* szOpPassword,
                                           IN TTBOOL bMakeOperator);

    
    TEAMTALKDLL_API INT32 TT_DoKickUser(IN TTInstance* lpTTInstance,
                                        IN INT32 nUserID,
                                        IN INT32 nChannelID);

    
    TEAMTALKDLL_API INT32 TT_DoSendFile(IN TTInstance* lpTTInstance,
                                        IN INT32 nChannelID,
                                        IN const TTCHAR* szLocalFilePath);

    
    TEAMTALKDLL_API INT32 TT_DoRecvFile(IN TTInstance* lpTTInstance,
                                        IN INT32 nChannelID,
                                        IN INT32 nFileID, 
                                        IN const TTCHAR* szLocalFilePath);

    
    TEAMTALKDLL_API INT32 TT_DoDeleteFile(IN TTInstance* lpTTInstance,
                                          IN INT32 nChannelID,
                                          IN INT32 nFileID);

    
    TEAMTALKDLL_API INT32 TT_DoSubscribe(IN TTInstance* lpTTInstance,
                                         IN INT32 nUserID, 
                                         IN Subscriptions uSubscriptions);

    
    TEAMTALKDLL_API INT32 TT_DoUnsubscribe(IN TTInstance* lpTTInstance,
                                           IN INT32 nUserID, 
                                           IN Subscriptions uSubscriptions);

    
    TEAMTALKDLL_API INT32 TT_DoMakeChannel(IN TTInstance* lpTTInstance,
                                           IN const Channel* lpChannel);

    
    TEAMTALKDLL_API INT32 TT_DoUpdateChannel(IN TTInstance* lpTTInstance,
                                             IN const Channel* lpChannel);

    
    TEAMTALKDLL_API INT32 TT_DoRemoveChannel(IN TTInstance* lpTTInstance,
                                             IN INT32 nChannelID);

    
    TEAMTALKDLL_API INT32 TT_DoMoveUser(IN TTInstance* lpTTInstance,
                                        IN INT32 nUserID, 
                                        IN INT32 nChannelID);

    
    TEAMTALKDLL_API INT32 TT_DoUpdateServer(IN TTInstance* lpTTInstance,
                                            IN const ServerProperties* lpServerProperties);

    
    TEAMTALKDLL_API INT32 TT_DoListUserAccounts(IN TTInstance* lpTTInstance,
                                                IN INT32 nIndex,
                                                IN INT32 nCount);

    
    TEAMTALKDLL_API INT32 TT_DoNewUserAccount(IN TTInstance* lpTTInstance,
                                              IN const UserAccount* lpUserAccount);

    
    TEAMTALKDLL_API INT32 TT_DoDeleteUserAccount(IN TTInstance* lpTTInstance,
                                                 IN const TTCHAR* szUsername);

    
    TEAMTALKDLL_API INT32 TT_DoBanUser(IN TTInstance* lpTTInstance,
                                       IN INT32 nUserID,
                                       IN INT32 nChannelID);

    
    TEAMTALKDLL_API INT32 TT_DoBanUserEx(IN TTInstance* lpTTInstance,
                                         IN INT32 nUserID,
                                         IN BanTypes uBanTypes);
    
    
    TEAMTALKDLL_API INT32 TT_DoBan(IN TTInstance* lpTTInstance,
                                   IN const BannedUser* lpBannedUser);

    
    TEAMTALKDLL_API INT32 TT_DoBanIPAddress(IN TTInstance* lpTTInstance,
                                            IN const TTCHAR* szIPAddress,
                                            IN INT32 nChannelID);

    
    TEAMTALKDLL_API INT32 TT_DoUnBanUser(IN TTInstance* lpTTInstance,
                                         IN const TTCHAR* szIPAddress,
                                         IN INT32 nChannelID);
    
    TEAMTALKDLL_API INT32 TT_DoUnBanUserEx(IN TTInstance* lpTTInstance,
                                           IN const BannedUser* lpBannedUser);

    
    TEAMTALKDLL_API INT32 TT_DoListBans(IN TTInstance* lpTTInstance,
                                        IN INT32 nChannelID,
                                        IN INT32 nIndex,
                                        IN INT32 nCount);

    
    TEAMTALKDLL_API INT32 TT_DoSaveConfig(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_DoQueryServerStats(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_DoQuit(IN TTInstance* lpTTInstance);
    

    
    
    
    TEAMTALKDLL_API TTBOOL TT_GetServerProperties(IN TTInstance* lpTTInstance,
                                                  OUT ServerProperties* lpServerProperties);

    
    TEAMTALKDLL_API TTBOOL TT_GetServerUsers(IN TTInstance* lpTTInstance,
                                             IN OUT User* lpUsers,
                                             IN OUT INT32* lpnHowMany);
    

    

    
    TEAMTALKDLL_API INT32 TT_GetRootChannelID(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_GetMyChannelID(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_GetChannel(IN TTInstance* lpTTInstance,
                                         IN INT32 nChannelID, 
                                         OUT Channel* lpChannel );
    
    
    TEAMTALKDLL_API TTBOOL TT_GetChannelPath(IN TTInstance* lpTTInstance,
                                             IN INT32 nChannelID, 
                                             OUT TTCHAR szChannelPath[TT_STRLEN]);

    
    TEAMTALKDLL_API INT32 TT_GetChannelIDFromPath(IN TTInstance* lpTTInstance,
                                                  IN const TTCHAR* szChannelPath);

    
    TEAMTALKDLL_API TTBOOL TT_GetChannelUsers(IN TTInstance* lpTTInstance,
                                            IN INT32 nChannelID,
                                            IN OUT User* lpUsers,
                                            IN OUT INT32* lpnHowMany);

    
    TEAMTALKDLL_API TTBOOL TT_GetChannelFiles(IN TTInstance* lpTTInstance,
                                              IN INT32 nChannelID, 
                                              IN OUT RemoteFile* lpRemoteFiles,
                                              IN OUT INT32* lpnHowMany);

    
    TEAMTALKDLL_API TTBOOL TT_GetChannelFile(IN TTInstance* lpTTInstance,
                                             IN INT32 nChannelID, 
                                             IN INT32 nFileID, 
                                             OUT RemoteFile* lpRemoteFile); 
    
    
    TEAMTALKDLL_API TTBOOL TT_IsChannelOperator(IN TTInstance* lpTTInstance,
                                                IN INT32 nUserID, 
                                                IN INT32 nChannelID);

    
    TEAMTALKDLL_API TTBOOL TT_GetServerChannels(IN TTInstance* lpTTInstance,
                                                IN OUT Channel* lpChannels,
                                                IN OUT INT32* lpnHowMany);
    

    

    
    TEAMTALKDLL_API INT32 TT_GetMyUserID(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_GetMyUserAccount(IN TTInstance* lpTTInstance,
                                               OUT UserAccount* lpUserAccount);
    
    
    TEAMTALKDLL_API UserTypes TT_GetMyUserType(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API UserRights TT_GetMyUserRights(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API INT32 TT_GetMyUserData(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_GetUser(IN TTInstance* lpTTInstance,
                                      IN INT32 nUserID, OUT User* lpUser);
    
    
    TEAMTALKDLL_API TTBOOL TT_GetUserStatistics(IN TTInstance* lpTTInstance,
                                                IN INT32 nUserID, 
                                                OUT UserStatistics* lpUserStatistics);
    
    TEAMTALKDLL_API TTBOOL TT_GetUserByUsername(IN TTInstance* lpTTInstance,
                                                IN const TTCHAR* szUsername, 
                                                OUT User* lpUser);
    

    

    
    TEAMTALKDLL_API TTBOOL TT_SetUserVolume(IN TTInstance* lpTTInstance,
                                            IN INT32 nUserID, 
                                            IN StreamType nStreamType,
                                            IN INT32 nVolume);

    
    TEAMTALKDLL_API TTBOOL TT_SetUserMute(IN TTInstance* lpTTInstance,
                                          IN INT32 nUserID,
                                          IN StreamType nStreamType,
                                          IN TTBOOL bMute);

    
    TEAMTALKDLL_API TTBOOL TT_SetUserStoppedPlaybackDelay(IN TTInstance* lpTTInstance,
                                                          IN INT32 nUserID, 
                                                          IN StreamType nStreamType,
                                                          IN INT32 nDelayMSec);

     
     TEAMTALKDLL_API TTBOOL TT_SetUserJitterControl(IN TTInstance* lpTTInstance,
                                                    IN INT32 nUserID,
                                                    IN StreamType nStreamType,
                                                    IN const JitterConfig* lpJitterConfig);

     
     TEAMTALKDLL_API TTBOOL TT_GetUserJitterControl(IN TTInstance* lpTTInstance,
                                                    IN INT32 nUserID,
                                                    IN StreamType nStreamType,
                                                    IN JitterConfig* lpJitterConfig);

     
    TEAMTALKDLL_API TTBOOL TT_SetUserPosition(IN TTInstance* lpTTInstance,
                                              IN INT32 nUserID, 
                                              IN StreamType nStreamType,
                                              IN float x,
                                              IN float y, 
                                              IN float z);

    
    TEAMTALKDLL_API TTBOOL TT_SetUserStereo(IN TTInstance* lpTTInstance,
                                            IN INT32 nUserID, 
                                            IN StreamType nStreamType,
                                            IN TTBOOL bLeftSpeaker, 
                                            IN TTBOOL bRightSpeaker);

    
    TEAMTALKDLL_API TTBOOL TT_SetUserMediaStorageDir(IN TTInstance* lpTTInstance,
                                                     IN INT32 nUserID,
                                                     IN const TTCHAR* szFolderPath,
                                                     IN const TTCHAR* szFileNameVars,
                                                     IN AudioFileFormat uAFF);

    
    TEAMTALKDLL_API TTBOOL TT_SetUserMediaStorageDirEx(IN TTInstance* lpTTInstance,
                                                       IN INT32 nUserID,
                                                       IN const TTCHAR* szFolderPath,
                                                       IN const TTCHAR* szFileNameVars,
                                                       IN AudioFileFormat uAFF,
                                                       IN INT32 nStopRecordingExtraDelayMSec);

    
    TEAMTALKDLL_API TTBOOL TT_SetUserAudioStreamBufferSize(IN TTInstance* lpTTInstance,
                                                           IN INT32 nUserID,
                                                           IN StreamTypes uStreamType,
                                                           IN INT32 nMSec);

    
    TEAMTALKDLL_API AudioBlock* TT_AcquireUserAudioBlock(IN TTInstance* lpTTInstance,
                                                         IN StreamTypes uStreamTypes,
                                                         IN INT32 nUserID);

    
    TEAMTALKDLL_API TTBOOL TT_ReleaseUserAudioBlock(IN TTInstance* lpTTInstance,
                                                    IN AudioBlock* lpAudioBlock);

    

    
    TEAMTALKDLL_API TTBOOL TT_GetFileTransferInfo(IN TTInstance* lpTTInstance,
                                                  IN INT32 nTransferID, 
                                                  OUT FileTransfer* lpFileTransfer);

    
    TEAMTALKDLL_API TTBOOL TT_CancelFileTransfer(IN TTInstance* lpTTInstance,
                                                 IN INT32 nTransferID);

    
    TEAMTALKDLL_API void TT_GetErrorMessage(IN INT32 nError, 
                                            OUT TTCHAR szErrorMsg[TT_STRLEN]);


    

    
    typedef enum TTKeyTranslate
    {
        
        TTKEY_NO_TRANSLATE                  = 0,
        
        TTKEY_WINKEYCODE_TO_TTKEYCODE       = 1,
        
        TTKEY_TTKEYCODE_TO_WINKEYCODE       = 2,
        
        TTKEY_MACKEYCODE_TO_TTKEYCODE       = 3,
        
        TTKEY_TTKEYCODE_TO_MACKEYCODE       = 4
    } TTKeyTranslate;

    
    TEAMTALKDLL_API INT32 TT_DesktopInput_KeyTranslate(TTKeyTranslate nTranslate,
                                                       IN const DesktopInput* lpDesktopInputs,
                                                       OUT DesktopInput* lpTranslatedDesktopInputs,
                                                       IN INT32 nDesktopInputCount);

    
    TEAMTALKDLL_API INT32 TT_DesktopInput_Execute(IN const DesktopInput* lpDesktopInputs,
                                                  IN INT32 nDesktopInputCount);

    

#if defined(WIN32)
    
    

    
    TEAMTALKDLL_API TTBOOL TT_HotKey_Register(IN TTInstance* lpTTInstance,
                                              IN INT32 nHotKeyID, 
                                              IN const INT32* lpnVKCodes,
                                              IN INT32 nVKCodeCount);

    
    TEAMTALKDLL_API TTBOOL TT_HotKey_Unregister(IN TTInstance* lpTTInstance,
                                                IN INT32 nHotKeyID);

    
    TEAMTALKDLL_API INT32 TT_HotKey_IsActive(IN TTInstance* lpTTInstance,
                                             IN INT32 nHotKeyID);

    
    TEAMTALKDLL_API TTBOOL TT_HotKey_InstallTestHook(IN TTInstance* lpTTInstance,
                                                     IN HWND hWnd, UINT32 uMsg);

    
    TEAMTALKDLL_API TTBOOL TT_HotKey_RemoveTestHook(IN TTInstance* lpTTInstance);

    
    TEAMTALKDLL_API TTBOOL TT_HotKey_GetKeyString(IN TTInstance* lpTTInstance,
                                                  IN INT32 nVKCode,
                                                  OUT TTCHAR szKeyName[TT_STRLEN]);
    
#endif

    
    TEAMTALKDLL_API INT32 TT_DBG_SIZEOF(IN TTType nType);

    TEAMTALKDLL_API VOID* TT_DBG_GETDATAPTR(IN TTMessage* pMsg);

    TEAMTALKDLL_API TTBOOL TT_DBG_SetSoundInputTone(IN TTInstance* lpTTInstance,
                                                    IN StreamTypes uStreamTypes,
                                                    IN INT32 nFrequency);

    TEAMTALKDLL_API TTBOOL TT_DBG_WriteAudioFileTone(IN const MediaFileInfo* lpMediaFileInfo,
                                                     IN INT32 nFrequency);

#if defined(WIN32) 

    

    
    typedef enum MixerControl
    {
        WAVEOUT_MASTER,
        WAVEOUT_WAVE,
        WAVEOUT_MICROPHONE,

        WAVEIN_MICROPHONE,
        WAVEIN_LINEIN,
        WAVEIN_WAVEOUT,
    } MixerControl;

    
    TEAMTALKDLL_API INT32 TT_Mixer_GetMixerCount(void);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_GetMixerName(IN INT32 nMixerIndex,
                                                 OUT TTCHAR szMixerName[TT_STRLEN]);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveInName(IN INT32 nWaveDeviceID,
                                                  OUT TTCHAR szMixerName[TT_STRLEN]);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveOutName(IN INT32 nWaveDeviceID,
                                                   OUT TTCHAR szMixerName[TT_STRLEN]);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveOutMute(IN INT32 nWaveDeviceID, 
                                                   IN MixerControl nControl, 
                                                   IN TTBOOL bMute);

    
    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveOutMute(IN INT32 nWaveDeviceID, 
                                                  IN MixerControl nControl);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveOutVolume(IN INT32 nWaveDeviceID, 
                                                     IN MixerControl nControl, 
                                                     IN INT32 nVolume);

    
    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveOutVolume(IN INT32 nWaveDeviceID, 
                                                    IN MixerControl nControl);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInSelected(IN INT32 nWaveDeviceID, 
                                                      IN MixerControl nControl);

    
    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInSelected(IN INT32 nWaveDeviceID, 
                                                     IN MixerControl nControl);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInVolume(IN INT32 nWaveDeviceID, 
                                                    IN MixerControl nControl, 
                                                    IN INT32 nVolume);

    
    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInVolume(IN INT32 nWaveDeviceID, 
                                                   IN MixerControl nControl);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInBoost(IN INT32 nWaveDeviceID, 
                                                   IN TTBOOL bEnable);
    
    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInBoost(IN INT32 nWaveDeviceID);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInMute(IN INT32 nWaveDeviceID, 
                                                  IN TTBOOL bEnable);

    
    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInMute(IN INT32 nWaveDeviceID);

    
    TEAMTALKDLL_API INT32 TT_Mixer_GetWaveInControlCount(IN INT32 nWaveDeviceID);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveInControlName(IN INT32 nWaveDeviceID, 
                                                         IN INT32 nControlIndex, 
                                                         OUT TTCHAR szDeviceName[TT_STRLEN]);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_SetWaveInControlSelected(IN INT32 nWaveDeviceID, 
                                                             IN INT32 nControlIndex);

    
    TEAMTALKDLL_API TTBOOL TT_Mixer_GetWaveInControlSelected(IN INT32 nWaveDeviceID, 
                                                             IN INT32 nControlIndex);
    

    

    
    TEAMTALKDLL_API TTBOOL TT_Firewall_IsEnabled(void);
    
    
    TEAMTALKDLL_API TTBOOL TT_Firewall_Enable(IN TTBOOL bEnable);

    
    TEAMTALKDLL_API TTBOOL TT_Firewall_AppExceptionExists(IN const TTCHAR* szExecutable);

    
    TEAMTALKDLL_API TTBOOL TT_Firewall_AddAppException(IN const TTCHAR* szName, 
                                                       IN const TTCHAR* szExecutable);
    
    
    TEAMTALKDLL_API TTBOOL TT_Firewall_RemoveAppException(IN const TTCHAR* szExecutable);
    

#endif 

#ifdef __cplusplus
}
#endif

#endif 

