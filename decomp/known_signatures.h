#include <stdint.h>

typedef int (*rmTask_t)(void);
typedef void (*rmTaskSimple_t)(void);
typedef int (*rmTaskArg_t)(void *);

// Placeholders for editor only; jump.h gets the real types from the SDK.
typedef struct _V5_Device *V5_DeviceT;
typedef uint32_t V5_DeviceType;
typedef uint32_t rmTaskStates;
typedef struct _rmSemaphore rmSemaphore;
typedef struct _V5_DeviceArmTipPosition V5_DeviceArmTipPosition;

// Signatures for jump table entries missing from a platform's SDK header.
// SDK declarations take precedence over these.
int32_t vexTasksRun();
void vexTaskAdd(rmTask_t, int interval, char const *label);

// From the IQ2 SDK (missing on V5 and EXP)
void vexTaskAddWithPriority(rmTask_t, int interval, char const *label,
                            int priority);
void vexTaskAddSimple(rmTaskSimple_t, int interval, char const *label);
void vexTaskAddSimpleWithPriority(rmTaskSimple_t, int interval,
                                  char const *label, int priority);
void vexTaskStop(rmTask_t);
void vexTaskSuspend(rmTask_t);
void vexTaskResume(rmTask_t);
void vexTaskSuspendCurrent(void);
void vexTaskResumeCurrent(void);
void vexTaskProgramSuspend(void);
void vexTaskProgramResume(void);
int32_t vexTaskPriorityGet(rmTask_t);
void vexTaskPrioritySet(rmTask_t, int32_t priority);
void vexTaskYield(void);
void vexTaskCheckTimeslice(void);
int32_t vexTaskGetIndex(void);
void vexTaskSleep(uint32_t time);
void vexSemaphoreInit(rmSemaphore *pSemaphore);
void vexSemaphoreLock(rmSemaphore *pSemaphore, uint32_t time);
void vexSemaphoreUnlock(rmSemaphore *pSemaphore);
int vexSemaphoreGetOwner(rmSemaphore *pSemaphore);
void vexTasksDump(void);
void *vexTaskGetCallback(uint32_t index);
void vexTaskWaitForExit(rmTask_t);
rmTaskStates vexTaskStateGet(rmTask_t);
int32_t vexTaskGetTaskIndex(rmTask_t);
void vexTaskStopAll(void);
void vexTaskStopAllUser(int (*maintask)(void));
void vexTaskRemoveAllUser(int (*maintask)(void));
int32_t vexTaskHardwareConcurrency(void);
void vexTaskCompletionIdSet(int32_t id);
int32_t vexTaskAddWithArg(rmTaskArg_t, int interval, void *arg,
                          char const *label);
int32_t vexTaskAddWithPriorityWithArg(rmTaskArg_t, int interval, void *arg,
                                      char const *label, int priority);
void vexTaskStopWithId(rmTaskArg_t, int callback_id);
void vexTaskSuspendWithId(rmTaskArg_t, int callback_id);
void vexTaskResumeWithId(rmTaskArg_t, int callback_id);
int32_t vexTaskPriorityGetWithId(rmTaskArg_t, int callback_id);
void vexTaskPrioritySetWithId(rmTaskArg_t, int callback_id, int32_t priority);
rmTaskStates vexTaskStateGetWithId(rmTaskArg_t, int callback_id);
int32_t vexTaskGetTaskIndexWithId(rmTaskArg_t, int callback_id);
void vexTaskWaitForExitWithId(rmTaskArg_t, int callback_id);
void *vexTaskGet(uint32_t index);
void *vexTaskGetCallbackAndId(uint32_t index, int *callback_id);
int32_t vexTaskStackSizeGet(void);
int32_t vexTaskStackDefaultSizeGet(void);
int32_t vexTaskStackUseGet(uint32_t index);
void *vexTaskStackTopGet(uint32_t index);
int32_t vexTaskFree(uint32_t index);
void vexTaskSetArgs(uint32_t index, void *args);
void vexCriticalSection(bool value);
void vexEventBroadcast(uint32_t index);
void vexEventBroadcastAndWait(uint32_t index, int32_t timeout);
void vexEventAdd(uint32_t index, uint32_t mask, void (*callback)(void));
int32_t vexEventUserIndexGet(void);
void vexEventAddWithArg(uint32_t index, uint32_t mask, void (*callback)(void *),
                        void *arg);
void vexEventsDump(void);
void vexEventsCleanup(void);
void *vexEventGetArg(void *event);
int32_t vexEventsGetMax(void);
int32_t vexEventsGetCount(void);
void vexBreak(uint32_t filehash, uint32_t tag);
void vexTaskBreakpointSet(uint32_t filehash, uint32_t tag);
void vexTaskBreakpointDump(void);
void *vexTaskGetArgs(uint32_t index, uint16_t *type);
V5_DeviceType vexDeviceTypeGetByIndex(uint32_t index);
void vexDeviceTypeSetByIndex(uint32_t index, V5_DeviceType type);
bool vexDeviceTimerSet(int32_t index, void (*callback)(void *), uint32_t delay);
bool vexDeviceTimerSetWithArg(int32_t index, void (*callback)(void *),
                              uint32_t delay, void *arg);
void vexDeviceTimerDump(void);
uint32_t vexDeviceFlagsGetByIndex(uint32_t index);
void vexDisplayTextSmoothing(uint32_t value);
void vexDisplayTextReference(uint32_t value);
void vexDisplayTextSpacing(uint32_t value);
void vexDisplayPenSizeSet(uint32_t value);
uint32_t vexDisplayPenSizeGet(void);
void vexSerialEnableRemoteConsole(void);
void vexDisplayClipRegionSetWithIndex(int32_t index, int32_t x1, int32_t y1,
                                      int32_t x2, int32_t y2);

// From the EXP SDK (missing on V5)
uint32_t vexDeviceLightTowerDebugGet(V5_DeviceT device, int32_t id);
void vexDeviceAiVisionSensorSet(V5_DeviceT device, double brightness,
                                double contrast);
void vexDeviceArmPoseSet(V5_DeviceT device, uint8_t pose, uint16_t velocity);
void vexDeviceArmMoveTipCommandLinear(V5_DeviceT device, int32_t x, int32_t y,
                                      int32_t z, uint8_t pose,
                                      uint16_t velocity, double j6_rotation,
                                      uint16_t j6_velocity, bool relative);
void vexDeviceArmMoveTipCommandJoint(V5_DeviceT device, int32_t x, int32_t y,
                                     int32_t z, uint8_t pose, uint16_t velocity,
                                     double j6_rotation, uint16_t j6_velocity,
                                     bool relative);
void vexDeviceArmMoveJointsCommand(V5_DeviceT device, double *positions,
                                   uint16_t *velocities, double j6_rotation,
                                   uint16_t j6_velocity, double j7_volts,
                                   uint16_t j7_timeout, uint16_t j7_i_limit,
                                   bool relative);
void vexDeviceArmSpinJoints(V5_DeviceT device, double *velocities);
void vexDeviceArmSetJointPositions(V5_DeviceT device, double *new_positions);
void vexDeviceArmPickUpCommand(V5_DeviceT device);
void vexDeviceArmDropCommand(V5_DeviceT device);
void vexDeviceArmMoveVoltsCommand(V5_DeviceT device, double *voltages);
void vexDeviceArmFullStop(V5_DeviceT device, uint8_t flags);
void vexDeviceArmEnableProfiler(V5_DeviceT device, uint8_t enable);
void vexDeviceArmProfilerVelocitySet(V5_DeviceT device,
                                     uint16_t linear_velocity,
                                     uint16_t joint_velocity);
void vexDeviceArmSaveZeroValues(V5_DeviceT device);
void vexDeviceArmForceZeroCommand(V5_DeviceT device);
void vexDeviceArmClearZeroValues(V5_DeviceT device);
void vexDeviceArmBootload(V5_DeviceT device);
void vexDeviceArmTipPositionGet(V5_DeviceT device, int32_t *x, int32_t *y,
                                int32_t *z);
void vexDeviceArmJointInfoGet(V5_DeviceT device, double *positions,
                              double *velocities, int32_t *currents);
double vexDeviceArmJ6PositionGet(V5_DeviceT device);
int32_t vexDeviceArmBatteryGet(V5_DeviceT device);
int32_t vexDeviceArmServoFlagsGet(V5_DeviceT device, uint32_t servoID);
uint32_t vexDeviceArmStatusGet(V5_DeviceT device);
uint32_t vexDeviceArmDebugGet(V5_DeviceT device, int32_t id);
void vexDeviceArmJointErrorsGet(V5_DeviceT device, uint8_t *errors);
void vexDeviceArmJ6PositionSet(V5_DeviceT device, int16_t position);
void vexDeviceArmStopJointsCommand(V5_DeviceT device, int16_t *brakeModes);
void vexDeviceArmReboot(V5_DeviceT device);
void vexDeviceArmTipOffsetSet(V5_DeviceT device, int32_t x, int32_t y,
                              int32_t z);
void vexDeviceArmMoveTipCommandLinearAdv(V5_DeviceT devicex,
                                         V5_DeviceArmTipPosition *position,
                                         double j6_rotation,
                                         uint16_t j6_velocity, bool relative);
void vexDeviceArmMoveTipCommandJointAdv(V5_DeviceT device,
                                        V5_DeviceArmTipPosition *position,
                                        double j6_rotation,
                                        uint16_t j6_velocity, bool relative);
void vexDeviceArmTipPositionGetAdv(V5_DeviceT device,
                                   V5_DeviceArmTipPosition *position);

// From the V5 SDK (vexDisplayRender missing on IQ2, radio missing on EXP)
bool vexDisplayRender(bool bVsyncWait, bool bRunScheduler);
void vexDeviceGenericRadioConnection(V5_DeviceT device, char *pName,
                                     bool bMaster, bool bAllowRadioOverride);
int32_t vexDeviceGenericRadioWriteChar(V5_DeviceT device, uint8_t c);
int32_t vexDeviceGenericRadioWriteFree(V5_DeviceT device);
int32_t vexDeviceGenericRadioTransmit(V5_DeviceT device, uint8_t *buffer,
                                      int32_t length);
int32_t vexDeviceGenericRadioReadChar(V5_DeviceT device);
int32_t vexDeviceGenericRadioPeekChar(V5_DeviceT device);
int32_t vexDeviceGenericRadioReceiveAvail(V5_DeviceT device);
int32_t vexDeviceGenericRadioReceive(V5_DeviceT device, uint8_t *buffer,
                                     int32_t length);
void vexDeviceGenericRadioFlush(V5_DeviceT device);
bool vexDeviceGenericRadioLinkStatus(V5_DeviceT device);
