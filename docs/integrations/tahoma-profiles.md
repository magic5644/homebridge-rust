# TaHoma declared profiles — P03 read-only evidence

Observed October 17, 2026 from the owner-run local `GET /setup` export. The owner reported execution; the ignored local file exists, is valid JSON (62,702 bytes, mode 0600), and contains 21 entries. No token was received by the agent. No raw payload, label, current state value, address, serial, device URL or gateway identifier is published here. These are reviewed technical declarations, not a physical-control test or a public fixture.

## Coverage and radio evidence

Four profiles cover the same family/widget counts as the 14 existing Homebridge physical-device projections: seven generic coverings, four Velux coverings, two Atlantic heaters and one garage. Each profile has identical definitions across its instances. Their controllable names and device URL schemes are all `io`; no RTS device is present in this snapshot. This supersedes the earlier assumption that the visible set necessarily contains RTS, without proving that no other RTS equipment exists outside the captured setup. Keep the fictional RTS example for synthetic P13 tests; obtain an actual RTS declaration before claiming real RTS support.

Seven additional infrastructure entries are not seven additional household actuators: one `internal:PodV2Component`, five `ogp:Bridge` entries, and one `io:StackComponent`. Do not publish/control those as HomeKit accessories or invoke their discovery, update, pairing or network-security commands. The nine manufacturer scenarios are separately evidenced by the existing plugin logs and owner-confirmed use; the local setup export has no action-group collection.

| Declared controllable | Count | uiClass / widgetName | Total declared commands |
|---|---:|---|---:|
| `io:RollerShutterGenericIOComponent` | 7 | RollerShutter / PositionableRollerShutter | 33 |
| `io:RollerShutterVeluxIOComponent` | 4 | RollerShutter / PositionableTiltedRollerShutter | 29 |
| `io:AtlanticElectricalHeaterIOComponent` | 2 | HeatingSystem / AtlanticElectricalHeater | 20 |
| `io:GarageOpenerIOComponent` | 1 | GarageDoor / PositionableGarageDoor | 27 |

Counts include maintenance/configuration commands, which are outside product control scope. The operational subset below records declarations only; it is not an execution allowlist. Parameter counts are advertised `nparams`; optional arguments in `paramsSig` do not establish bounds or units. Physical product references and numeric bounds/steps are absent from these definitions. `controllableName` is the technical profile, not an identified motor part number.

## Coverings

Both covering profiles declare `open(0)`, `close(0)`, `stop(0)`, `up(0)`, `down(0)`, `my(0)`, `setClosure(1)`, `setPosition(1)`, `setDeployment(1)`, `setMemorized1Position(1)`, `refreshMemorized1Position(0)` and `setSecuredPosition(1)`. The Velux profile additionally declares `setClosureAndLinearSpeed(1)` and `setPositionAndLinearSpeed(1)`, each with an optional second argument in `paramsSig`. Do not add speed controls without separate semantics/bounds evidence.

Generic profile state declarations include `core:ClosureState`, `core:TargetClosureState`, `core:OpenClosedState`, `core:MovingState`, `core:Memorized1PositionState`, `core:SecuredPositionState`, status/radio/lock states and `core:NameState`. The capture contains samples for closure, target closure, open/closed and moving state. Their values are intentionally not published.

Velux profile declarations include `core:ClosureState`, `core:OpenClosedState`, `core:BatteryDiscreteLevelState`, `core:Memorized1PositionState`, `core:SecuredPositionState`, status/radio/lock states and `core:NameState`. No target-closure or moving-state declaration appears for this profile. Neither covering profile declares a tilt/orientation command or state: the widget name alone does not establish tilt support. P12 must distinguish both profiles and verify closure/openness direction and bounds before mapping; P13 must not classify them as RTS.

## Atlantic heating

Operational declarations: `off(0)`, `setHeatingLevel(1)`, `setHeatingLevelForTrigger(1)`, `setHeatingLevelWithTimer(2)`, `cancelHeatingLevel(0)` with an optional argument, and `refreshHeatingLevel(0)`.

State declarations include `io:LastTargetHeatingLevelState`, `io:TargetHeatingLevelState`, `core:TargetHeatingLevelState`, `io:MaximumHeatingLevelState`, `io:TimerForTransitoryStateState`, `core:OnOffState` and status/radio/lock/name states. The capture contains target-level, maximum-level, timer and on/off samples. No temperature-measurement state, numeric setpoint command or declared temperature bounds are present. Mode values, timer units and cancellation behavior still require qualification at P34; do not turn the plugin's conventional temperatures into measured values or infer cooling from its HomeKit COOL projection.

## Garage

The actual profile declares separate `open(0)`, `close(0)`, `stop(0)`, `up(0)`, `down(0)` and `setClosure(1)`; it does **not** declare `cycle`. Its state declarations include `core:ClosureState`, `core:OpenClosedUnknownState` and status/radio/lock/name states; closure and open/closed/unknown samples are present. These are stronger evidence than a generic HomeKit door projection, but do not establish sensor reliability or safe physical behavior.

No explicit obstruction state is declared. P33 must verify whether status data conveys obstruction, validate feedback and safety with the owner, and preserve unknown when evidence is absent. Do not expose a fabricated obstruction=false or equate command completion with a closed door. No garage movement, obstruction test or command replay occurred during P03.

## Downstream limits

P03's raw-inventory prerequisite is now satisfied. P11/P12/P13/P33/P34 consume these declared profiles and explicit absences; they must obtain any missing bounds, units, events/error shapes or additional profiles before implementing a corresponding claim. P35 still qualifies each stored scenario's exact targets and trigger/reset/cancel behavior; keep its existing Homebridge path until validated. This is allowed by P03's requirement to record missing profiles as downstream prerequisites, not declared support.

The ten public fixtures remain authored fictional examples, unchanged by this private capture. No event listener or command endpoint was called. Resource comparison remains deferred to P31/P38. Native hardware compatibility, scenario parity, controller versions, actual Matter use and kernel compatibility have not been demonstrated.
