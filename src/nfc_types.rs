
struct NFCContext;
struct NFCDriver;
struct NFCConnString;

//type NFCConnString = [char; 1024];

struct NBR_106;
struct NBR_212;
struct NBR_424;
struct NBR_847;

enum NFCBaudRate {
    NBR_UNDEFINED = 0,
    NBR_106,
    NBR_212,
    NBR_424,
    NBR_847,
}

struct NFCModulation {
    NMT: NFCModulationType,
    NBR: NFCBaudRate,
}
