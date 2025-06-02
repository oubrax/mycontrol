# GPUI Rules and Guidelines

This document summarizes key concepts and guidelines for working with the `gpui` crate, a GPU-accelerated UI framework for Rust.

## Overview

- Purpose: GPUI is a hybrid immediate and retained mode, GPU accelerated UI framework for Rust, designed for a wide variety of applications.
- Development Status: Actively developed, not yet on crates.io. Requires latest stable Rust, supports macOS and Linux.
- Build: Platform-specific build steps (Metal on macOS, resources on Windows) and shader compilation (WGSL, Metal) are part of the build process (build.rs).

## Core Concepts

- Application: The entry point of a GPUI application (struct Application).
    - new() -> Self
    - headless() -> Self
    - with_assets(self, asset_source: impl AssetSource) -> Self
    - with_http_client(self, http_client: Arc<dyn HttpClient>) -> Self
    - run<F>(self, on_finish_launching: F)
    - on_open_urls<F>(&self, mut callback: F) -> &Self
    - on_reopen<F>(&self, mut callback: F) -> &Self
    - background_executor(&self) -> BackgroundExecutor
    - foreground_executor(&self) -> ForegroundExecutor
    - text_system(&self) -> Arc<TextSystem>
    - path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf>

- Entities: Entity<T> is the primary mechanism for state management and communication.
    - update<R>(&self, cx: &mut App, update: impl FnOnce(&mut T, &mut Context<T>) -> R) -> R
    - read<R>(&self, cx: &App) -> &T
    - read_with<R>(&self, cx: &App, read: impl FnOnce(&T, &App) -> R) -> R
    - downgrade(&self) -> WeakEntity<T>
    - entity_id(&self) -> EntityId
    - AnyEntity: Dynamically-typed entity handle.
    - WeakEntity<T>: Weak handle to an entity.
        - upgrade(&self) -> Option<Entity<T>>
    - AnyWeakEntity: Dynamically-typed weak entity handle.
        - upgrade(&self) -> Option<AnyEntity>
    - EventEmitter<E>: Trait for entities that can emit events.
    - Reservation<T>: Allows reserving an EntityId before creating the entity.
        - entity_id(&self) -> EntityId

- Views: High-level, declarative UI components. An Entity that implements the Render trait.
    - AnyView, AnyWeakView: Dynamically-typed view handles.
        - AnyView::cached(mut self, style: StyleRefinement) -> Self
        - AnyView::downgrade(&self) -> AnyWeakView
        - AnyView::downcast<T: 'static>(self) -> Result<Entity<T>, Self>
        - AnyView::entity_type(&self) -> TypeId
        - AnyView::entity_id(&self) -> EntityId
        - AnyWeakView::upgrade(&self) -> Option<AnyView>
    - Focusable: Trait for views that can receive focus.
        - focus_handle(&self, cx: &App) -> FocusHandle
    - ManagedView: Trait for views whose lifecycle is managed by another view (e.g., modals). Emits DismissEvent.

- Elements: Low-level, imperative UI building blocks. Implement the Element trait.
    - Element::id(&self) -> Option<ElementId>
    - Element::source_location(&self) -> Option<&'static panic::Location<'static>>
    - Element::request_layout(&mut self, id: Option<&GlobalElementId>, inspector_id: Option<&InspectorElementId>, window: &mut Window, cx: &mut App) -> (LayoutId, Self::RequestLayoutState)
    - Element::prepaint(&mut self, id: Option<&GlobalElementId>, inspector_id: Option<&InspectorElementId>, bounds: Bounds<Pixels>, request_layout: &mut Self::RequestLayoutState, window: &mut Window, cx: &mut App) -> Self::PrepaintState
    - Element::paint(&mut self, id: Option<&GlobalElementId>, inspector_id: Option<&InspectorElementId>, bounds: Bounds<Pixels>, request_layout: &mut Self::RequestLayoutState, prepaint: &mut Self::PrepaintState, window: &mut Window, cx: &mut App)
    - Element::into_any(self) -> AnyElement
    - IntoElement: Trait for types convertible to an element.
        - into_element(self) -> Self::Element
        - into_any_element(self) -> AnyElement
    - Render: Trait for views that can be rendered into an element tree.
        - render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement
    - RenderOnce: Trait for components that render once and are consumed. Use #[derive(IntoElement)].
        - render(self, window: &mut App) -> impl IntoElement
    - ParentElement: Trait for elements that can contain children.
        - extend(&mut self, elements: impl IntoIterator<Item = AnyElement>)
        - child(mut self, child: impl IntoElement) -> Self
        - children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
    - GlobalElementId: Unique identifier for tracking elements across frames.
    - AnyElement: Dynamically-typed element.

- Memory Management: Elements are allocated in a custom Arena.
    - Arena::new(size_in_bytes: usize) -> Self
    - Arena::len(&self) -> usize
    - Arena::capacity(&self) -> usize
    - Arena::clear(&mut self)
    - Arena::alloc<T>(&mut self, f: impl FnOnce() -> T) -> ArenaBox<T>
    - ArenaBox<T>: Pointer to arena-allocated data.
        - map<U: ?Sized>(mut self, f: impl FnOnce(&mut T) -> &mut U) -> ArenaBox<U>
    - ArenaRef<T>: Reference to arena-allocated data.
        - from(value: ArenaBox<T>) -> Self

## Contexts

Context types provide access to global state, windows, entities, and system services. Passed as the cx argument.

- AppContext: Trait for application contexts.
    - new<T: 'static>(&mut self, build_entity: impl FnOnce(&mut Context<T>) -> T) -> Self::Result<Entity<T>>
    - reserve_entity<T: 'static>(&mut self) -> Self::Result<Reservation<T>>
    - insert_entity<T: 'static>(&mut self, reservation: Reservation<T>, build_entity: impl FnOnce(&mut Context<T>) -> T) -> Self::Result<Entity<T>>
    - update_entity<T, R>(&mut self, handle: &Entity<T>, update: impl FnOnce(&mut T, &mut Context<T>) -> R) -> Self::Result<R>
    - read_entity<T, R>(&self, handle: &Entity<T>, read: impl FnOnce(&T, &App) -> R) -> Self::Result<R>
    - update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    - read_window<T, R>(&self, window: &WindowHandle<T>, read: impl FnOnce(Entity<T>, &App) -> R) -> Result<R>
    - background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    - read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> Self::Result<R>

- VisualContext: Trait for contexts requiring a window.
    - window_handle(&self) -> AnyWindowHandle
    - update_window_entity<T: 'static, R>(&mut self, entity: &Entity<T>, update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R) -> Self::Result<R>
    - new_window_entity<T: 'static>(&mut self, build_entity: impl FnOnce(&mut Window, &mut Context<T>) -> T) -> Self::Result<Entity<T>>
    - replace_root_view<V>(&mut self, build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V) -> Self::Result<Entity<V>>
    - focus<V>(&mut self, entity: &Entity<V>) -> Self::Result<()>

- BorrowAppContext: Helper trait for global state access.
    - set_global<T: Global>(&mut self, global: T)
    - update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    - update_default_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R

- App: The main application state struct.
    - shutdown(&mut self)
    - keyboard_layout(&self) -> &dyn PlatformKeyboardLayout
    - on_keyboard_layout_change<F>(&self, mut callback: F) -> Subscription
    - quit(&self)
    - refresh_windows(&mut self)
    - observe<W>(&mut self, entity: &Entity<W>, mut on_notify: impl FnMut(Entity<W>, &mut App) + 'static) -> Subscription
    - subscribe<T, Event>(&mut self, emitter: &Entity<T>, mut on_event: impl FnMut(&mut Self, Entity<T>, &Event, &mut Context<T>) + 'static) -> Subscription
    - windows(&self) -> Vec<AnyWindowHandle>
    - open_window<V: 'static + Render>(&mut self, options: WindowOptions, build_root_view: impl FnOnce(&mut Window, &mut Context<V>) -> V) -> Result<WindowHandle<V>>
    - screen_capture_sources(&self) -> Result<Vec<ScreenCaptureSource>>
    - find_display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>>
    - write_credentials(&self, url: Url, username: &str, password: &[u8]) -> Result<()>
    - prompt_for_paths(&self, options: PathPromptOptions) -> Result<Option<Vec<PathBuf>>>
    - prompt_for_new_path(&self, options: PathPromptOptions) -> Result<Option<PathBuf>>
    - to_async(&self) -> AsyncApp
    - defer(&mut self, f: impl FnOnce(&mut App) + 'static)
    - global<G: Global>(&self) -> &G
    - try_global<G: Global>(&self) -> Option<&G>
    - global_mut<G: Global>(&mut self) -> &mut G
    - default_global<G: Global + Default>(&mut self) -> &mut G
    - set_global<G: Global>(&mut self, global: G)
    - remove_global<G: Global>(&mut self) -> G
    - observe_global<G: Global>(&mut self, mut on_notify: impl FnMut(&mut App) + 'static) -> Subscription
    - observe_new<T: 'static>(&mut self, mut on_new: impl FnMut(Entity<T>, &mut Option<&mut Window>, &mut App) + 'static) -> Subscription
    - observe_release<T>(&mut self, entity: &Entity<T>, mut on_release: impl FnOnce(&mut T, &mut App) + 'static) -> Subscription
    - observe_release_in<T>(&mut self, entity: &Entity<T>, mut on_release: impl FnOnce(&mut T, &mut Window, &mut Context<T>) + 'static) -> Subscription
    - observe_keystrokes(&mut self, mut on_keystroke: impl FnMut(&KeystrokeEvent, &mut Window, &mut App) -> bool + 'static) -> Subscription
    - bind_keys(&mut self, bindings: impl IntoIterator<Item = KeyBinding>)
    - clear_key_bindings(&mut self)
    - on_action<A: Action>(&mut self, listener: impl Fn(&A, &mut Self) + 'static)
    - on_app_quit<Fut>(&mut self, on_quit: impl FnOnce(&mut App) -> Fut + 'static) -> Subscription
    - on_window_closed(&self, mut on_closed: impl FnMut(&mut App) + 'static) -> Subscription
    - is_action_available(&mut self, action: &dyn Action) -> bool
    - update_jump_list(&self, items: Vec<crate::JumpListItem>)
    - dispatch_action(&mut self, action: &dyn Action)
    - stop_active_drag(&mut self, window: &mut Window) -> bool
    - set_prompt_builder(&mut self, prompt_builder: PromptBuilder)
    - remove_asset<A: Asset>(&mut self, source: &A::Source)
    - fetch_asset<A: Asset>(&mut self, source: &A::Source) -> (Shared<Task<A::Output>>, bool)
    - notify(&mut self, entity_id: EntityId)
    - drop_image(&mut self, image: Arc<RenderImage>, current_window: Option<&mut Window>)

- Window: Represents an application window.
    - invalidate_view(&self, view_id: EntityId, cx: &mut App) -> bool
    - observe_window_appearance(&self, mut on_change: impl FnMut(&mut App) + 'static) -> Subscription
    - replace_root<E>(&mut self, build_root_view: impl FnOnce(&mut Window, &mut Context<E>) -> E) -> Result<Entity<E>>
    - root<E>(&self) -> Option<Option<Entity<E>>>
    - refresh(&mut self)
    - focused(&self, cx: &App) -> Option<FocusHandle>
    - focus(&mut self, handle: &FocusHandle)
    - blur(&mut self)
    - disable_focus(&mut self)
    - text_style(&self) -> TextStyle
    - dispatch_action(&mut self, action: Box<dyn Action>, cx: &mut App)
    - defer(&self, cx: &mut App, f: impl FnOnce(&mut Window, &mut App) + 'static) -> Subscription
    - observe<T: 'static>(&mut self, entity: &Entity<T>, mut on_notify: impl FnMut(Entity<T>, &mut Window, &mut Context<T>) + 'static) -> Subscription
    - subscribe<Emitter, Evt>(&mut self, emitter: &Entity<Emitter>, mut on_event: impl FnMut(&mut Window, &Entity<Emitter>, &Evt, &mut Context<Emitter>) + 'static) -> Subscription
    - observe_release<T>(&mut self, entity: &Entity<T>, mut on_release: impl FnOnce(&mut T, &mut Window, &mut Context<T>) + 'static) -> Subscription
    - request_animation_frame(&self)
    - spawn<AsyncFn, R>(&self, cx: &App, f: AsyncFn) -> Task<R>
    - is_window_hovered(&self) -> bool
    - set_client_inset(&mut self, inset: Pixels)
    - set_background_appearance(&self, background_appearance: WindowBackgroundAppearance)
    - display(&self, cx: &App) -> Option<Rc<dyn PlatformDisplay>>
    - rem_size(&self) -> Pixels
    - with_global_id<R>(&mut self, id: ElementId, f: impl FnOnce(&mut Self) -> R) -> R
    - with_rem_size<F, R>(&mut self, rem_size: Option<impl Into<Pixels>>, f: F) -> R
    - is_action_available(&self, action: &dyn Action, cx: &App) -> bool
    - draw(&mut self, cx: &mut App)
    - present(&self)
    - with_text_style<F, R>(&mut self, style: Option<TextStyleRefinement>, f: F) -> R
    - set_cursor_style(&mut self, style: CursorStyle, hitbox: Option<&Hitbox>)
    - set_tooltip(&mut self, tooltip: AnyTooltip) -> TooltipId
    - with_content_mask<R>(&mut self, mask: ContentMask<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - with_element_offset<R>(&mut self, offset: Point<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - with_absolute_element_offset<R>(&mut self, offset: Point<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - with_element_opacity<R>(&mut self, opacity: f32, f: impl FnOnce(&mut Self) -> R) -> R
    - element_offset(&self) -> Point<Pixels>
    - element_opacity(&self) -> f32
    - content_mask(&self) -> ContentMask<Pixels>
    - with_element_namespace<R>(&mut self, namespace: ElementId, f: impl FnOnce(&mut Self) -> R) -> R
    - with_element_state<S, R>(&mut self, id: &GlobalElementId, f: impl FnOnce(Option<&mut S>, &mut Self) -> (R, S)) -> R
    - with_optional_element_state<S, R>(&mut self, id: &GlobalElementId, f: impl FnOnce(Option<&mut S>, &mut Self) -> (R, Option<S>)) -> R
    - defer_draw(&mut self, current_view: EntityId, priority: usize, element: impl IntoElement)
    - paint_layer<R>(&mut self, bounds: Bounds<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - paint_shadows(&mut self, shadows: &[BoxShadow], bounds: Bounds<Pixels>)
    - paint_quad(&mut self, quad: PaintQuad)
    - paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Background>)
    - paint_underline(&mut self, underline: &Underline, bounds: Bounds<Pixels>)
    - paint_strikethrough(&mut self, strikethrough: &StrikethroughStyle, bounds: Bounds<Pixels>)
    - paint_glyph(&mut self, origin: Point<Pixels>, glyph_id: GlyphId, font_id: FontId, font_size: Pixels, color: Hsla)
    - paint_emoji(&mut self, origin: Point<Pixels>, glyph_id: GlyphId, font_id: FontId, font_size: Pixels)
    - paint_svg(&mut self, params: RenderSvgParams)
    - paint_image(&mut self, params: RenderImageParams)
    - paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVPixelBuffer)
    - drop_image(&mut self, data: Arc<RenderImage>) -> Result<()>
    - request_layout(&mut self, style: Style, children: impl IntoIterator<Item = LayoutId>, cx: &mut App) -> LayoutId
    - request_measured_layout<T>(&mut self, style: Style, measure_func: taffy::MeasureFunc, cx: &mut App) -> LayoutId
    - compute_layout(&mut self, layout_id: LayoutId, available_space: Size<AvailableSpace>, cx: &mut App) -> Size<Pixels>
    - layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels>
    - insert_hitbox(&mut self, bounds: Bounds<Pixels>, opaque: bool) -> Hitbox
    - set_key_context(&mut self, context: KeyContext)
    - set_focus_handle(&mut self, focus_handle: &FocusHandle, _: &App)
    - set_view_id(&mut self, view_id: EntityId)
    - current_view(&self) -> EntityId
    - with_rendered_view<R>(&mut self, view_id: EntityId, f: impl FnOnce(&mut Self) -> R) -> R
    - with_image_cache<F, R>(&mut self, image_cache: Option<AnyImageCache>, f: F) -> R
    - handle_input(&mut self, handler: impl InputHandler + 'static)
    - on_mouse_event<Event: MouseEvent>(&mut self, listener: impl FnMut(&Event, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - on_key_event<Event: KeyEvent>(&mut self, listener: impl FnMut(&Event, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - on_modifiers_changed(&mut self, listener: impl FnMut(&ModifiersChangedEvent, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - on_focus_in(&mut self, listener: impl FnMut(&WindowFocusEvent, &mut Window, &mut App) + 'static) -> Subscription
    - on_focus_out(&mut self, listener: impl FnMut(&FocusOutEvent, &mut Window, &mut App) + 'static) -> Subscription
    - dispatch_keystroke(&mut self, keystroke: Keystroke, cx: &mut App) -> bool
    - keystroke_text_for(&self, action: &dyn Action) -> String
    - dispatch_event(&mut self, event: PlatformInput, cx: &mut App) -> DispatchEventResult
    - pending_input_keystrokes(&self) -> Option<&[Keystroke]>
    - observe_global<G: Global>(&mut self, mut on_notify: impl FnMut(&mut Window, &mut App) + 'static) -> Subscription
    - invalidate_character_coordinates(&self)
    - prompt(&mut self, level: PromptLevel, options: impl Into<PromptOptions>) -> RenderablePromptHandle
    - context_stack(&self) -> Vec<KeyContext>
    - available_actions(&self, cx: &App) -> Vec<Box<dyn Action>>
    - bindings_for_action(&self, action: &dyn Action) -> Vec<KeyBinding>
    - bindings_for_action_in(&self, action: &dyn Action, context: &[KeyContext]) -> Vec<KeyBinding>
    - bindings_for_action_in_context(&self, action: &dyn Action, context: &KeyContext) -> Vec<KeyBinding>
    - on_window_should_close(&self, mut on_should_close: impl FnMut(&mut App) -> bool + 'static) -> Subscription
    - on_action(&mut self, listener: impl Fn(&dyn Action, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - toggle_inspector(&mut self, cx: &mut App)
    - is_inspector_picking(&self, _cx: &App) -> bool
    - with_inspector_state<T: 'static, R>(&mut self, f: impl FnOnce(Option<&mut T>, &mut Window, &mut App) -> (R, Option<T>)) -> R

## Concurrency

- Executors:
    - BackgroundExecutor: Runs tasks on background threads.
        - new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self
        - spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
        - spawn_labeled<R>(&self, label: TaskLabel, future: impl Future<Output = R> + Send + 'static) -> Task<R>
        - block_test<R>(&self, future: impl Future<Output = R>) -> R
        - block<R>(&self, future: impl Future<Output = R>) -> R
        - block_with_timeout<Fut: Future>(&self, duration: Duration, future: Fut) -> Result<Fut::Output, impl Future<Output = Fut::Output> + use<Fut>>
        - scoped<'scope, F>(&self, scheduler: F)
        - now(&self) -> Instant
        - timer(&self, duration: Duration) -> Task<()>
        - num_cpus(&self) -> usize
        - is_main_thread(&self) -> bool
    - ForegroundExecutor: Runs tasks on the main thread. !Send.
        - new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self
        - spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>

- Tasks: Task<R> represents an async operation. Implements Future.
    - ready(val: T) -> Self
    - detach(self)
    - detach_and_log_err(self, cx: &App)
    - TaskLabel: Opaque identifier for a task in tests.
        - new() -> Self

## Input Handling

- Actions: User-defined structs for keyboard-driven UI. Implement the Action trait.
    - Action::boxed_clone(&self) -> Box<dyn Action>
    - Action::partial_eq(&self, action: &dyn Action) -> bool
    - Action::name(&self) -> &str
    - Action::debug_name() -> &'static str
    - Action::build(value: serde_json::Value) -> Result<Box<dyn Action>>
    - Action::action_json_schema(_: &mut schemars::r#gen::SchemaGenerator) -> Option<schemars::schema::Schema>
    - Action::deprecated_aliases() -> &'static [&'static str]
    - ActionRegistry: Manages registered actions.
        - build_action_type(&self, type_id: &TypeId) -> Result<Box<dyn Action>>
        - build_action(&self, name: &str, params: Option<serde_json::Value>) -> std::result::Result<Box<dyn Action>, ActionBuildError>
        - all_action_names(&self) -> &[SharedString]
        - action_schemas(&self, generator: &mut schemars::r#gen::SchemaGenerator) -> Vec<(SharedString, Option<schemars::schema::Schema>)>
        - action_deprecations(&self) -> &HashMap<SharedString, SharedString>
    - ActionBuildError: Error type for building actions.

- Keymaps: Keymap manages KeyBindings.
    - Keymap::new(bindings: Vec<KeyBinding>) -> Self
    - Keymap::version(&self) -> KeymapVersion
    - Keymap::add_bindings<T: IntoIterator<Item = KeyBinding>>(&mut self, bindings: T)
    - Keymap::clear(&mut self)
    - Keymap::bindings(&self) -> impl DoubleEndedIterator<Item = &KeyBinding>
    - Keymap::bindings_for_action<'a>(&'a self, action: &'a dyn Action) -> impl 'a + DoubleEndedIterator<Item = &'a KeyBinding>
    - Keymap::all_bindings_for_input(&self, input: &[Keystroke]) -> Vec<KeyBinding>
    - Keymap::bindings_for_input(&self, input: &[Keystroke], context_stack: &[KeyContext]) -> (SmallVec<[KeyBinding; 1]>, bool)
    - Keymap::binding_to_display_from_bindings(mut bindings: Vec<KeyBinding>) -> Option<KeyBinding>
    - Keymap::default_binding_from_bindings_iterator<'a>(mut bindings: impl Iterator<Item = &'a KeyBinding>) -> Option<&'a KeyBinding>
    - KeyBinding: Keystroke + Action + ContextPredicate.
    - KeyContext: Represents the context in which a key binding is active.
    - KeymapVersion: Opaque identifier for the keymap version.

- Input Handling for Views:
    - EntityInputHandler: Trait for views that handle text input.
        - text_for_range(&mut self, range: Range<usize>, adjusted_range: &mut Option<Range<usize>>, window: &mut Window, cx: &mut Context<Self>) -> Option<String>
        - selected_text_range(&mut self, ignore_disabled_input: bool, window: &mut Window, cx: &mut Context<Self>) -> Option<UTF16Selection>
        - marked_text_range(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<Range<usize>>
        - unmark_text(&mut self, window: &mut Window, cx: &mut Context<Self>)
        - replace_text_in_range(&mut self, range: Option<Range<usize>>, text: &str, window: &mut Window, cx: &mut Context<Self>)
        - replace_and_mark_text_in_range(&mut self, range: Option<Range<usize>>, new_text: &str, new_selected_range: Option<Range<usize>>, window: &mut Window, cx: &mut Context<Self>)
        - bounds_for_range(&mut self, range_utf16: Range<usize>, element_bounds: Bounds<Pixels>, window: &mut Window, cx: &mut Context<Self>) -> Option<Bounds<Pixels>>
        - character_index_for_point(&mut self, point: crate::Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) -> Option<usize>
    - ElementInputHandler<V>: Concrete implementation used with Window::handle_input.
        - new(element_bounds: Bounds<Pixels>, view: Entity<V>) -> Self

- Events: Mouse events, Keyboard events, FileDropEvent, ModifiersChangedEvent.
    - DispatchPhase: Capture and bubble phases of event dispatch.
    - Window::on_mouse_event<Event: MouseEvent>(&mut self, listener: impl FnMut(&Event, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - Window::on_key_event<Event: KeyEvent>(&mut self, listener: impl FnMut(&Event, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - Window::on_modifiers_changed(&mut self, listener: impl FnMut(&ModifiersChangedEvent, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - Window::on_focus_in(&mut self, listener: impl FnMut(&WindowFocusEvent, &mut Window, &mut App) + 'static) -> Subscription
    - Window::on_focus_out(&mut self, listener: impl FnMut(&FocusOutEvent, &mut Window, &mut App) + 'static) -> Subscription

- Focus:
    - FocusHandle, WeakFocusHandle: Handles for managing focus.
        - FocusHandle::new(handles: &Arc<FocusMap>) -> Self
        - FocusHandle::for_id(id: FocusId, handles: &Arc<FocusMap>) -> Option<Self>
        - FocusHandle::downgrade(&self) -> WeakFocusHandle
        - FocusHandle::focus(&self, window: &mut Window)
        - FocusHandle::is_focused(&self, window: &Window) -> bool
        - FocusHandle::contains_focused(&self, window: &Window, cx: &App) -> bool
        - FocusHandle::within_focused(&self, window: &Window, cx: &mut App) -> bool
        - FocusHandle::contains(&self, other: &Self, window: &Window) -> bool
        - FocusHandle::dispatch_action(&self, action: &dyn Action, window: &mut Window, cx: &mut App)
        - WeakFocusHandle::upgrade(&self) -> Option<FocusHandle>
    - FocusId: Unique identifier for a focusable element.
        - is_focused(&self, window: &Window) -> bool
        - contains_focused(&self, window: &Window, cx: &App) -> bool
        - within_focused(&self, window: &Window, cx: &App) -> bool

- Hit Testing: Hitbox defines a clickable region.
    - Window::insert_hitbox(&mut self, bounds: Bounds<Pixels>, opaque: bool) -> Hitbox
    - HitboxId::is_hovered(&self, window: &Window) -> bool
    - Hitbox::is_hovered(&self, window: &Window) -> bool

- Tooltips:
    - Window::set_tooltip(&mut self, tooltip: AnyTooltip) -> TooltipId
    - TooltipId::is_hovered(&self, window: &Window) -> bool

## Styling and Layout

- Styling: Style struct defines visual properties. Uses a Tailwind-like API.
    - Style::has_opaque_background(&self) -> bool
    - Style::text_style(&self) -> Option<&TextStyleRefinement>
    - Style::overflow_mask(&self, bounds: Bounds<Pixels>, content_mask: ContentMask<Pixels>) -> ContentMask<Pixels>
    - TextStyle: Defines text-specific styling.
        - highlight(mut self, style: impl Into<HighlightStyle>) -> Self
        - font(&self) -> Font
        - line_height_in_pixels(&self, rem_size: Pixels) -> Pixels
        - to_run(&self, len: usize) -> TextRun
    - HighlightStyle: Used for applying highlights to text.
        - color(color: Hsla) -> Self
        - highlight(&mut self, other: HighlightStyle)
    - ObjectFit: How images fit within bounds.
        - get_bounds(&self, bounds: Bounds<Pixels>, image_size: Size<DevicePixels>) -> Bounds<Pixels>
    - Fill: Background fill (color or image).
        - color(&self) -> Option<Background>
    - Free functions:
        - combine_highlights(highlights: impl IntoIterator<Item = HighlightStyle>) -> HighlightStyle
        - quad(bounds: impl Into<Bounds<Pixels>>, style: Style) -> PaintQuad
        - fill(bounds: impl Into<Bounds<Pixels>>, background: impl Into<Background>) -> PaintQuad
        - outline(bounds: impl Into<Bounds<Pixels>>, border_widths: impl Into<Edges<Pixels>>, border_color: impl Into<Hsla>, border_style: BorderStyle) -> PaintQuad

- Layout: Handled by Taffy.
    - Window::request_layout(&mut self, style: Style, children: impl IntoIterator<Item = LayoutId>, cx: &mut App) -> LayoutId
    - Window::request_measured_layout<T>(&mut self, style: Style, measure_func: taffy::MeasureFunc, cx: &mut App) -> LayoutId
    - Window::compute_layout(&mut self, layout_id: LayoutId, available_space: Size<AvailableSpace>, cx: &mut App) -> Size<Pixels>
    - Window::layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels>

- Content Masking: ContentMask defines the visible region.
    - Window::with_content_mask<R>(&mut self, mask: ContentMask<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - Window::content_mask(&self) -> ContentMask<Pixels>

- Element Transforms: Apply transforms during painting.
    - Window::with_element_offset<R>(&mut self, offset: Point<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - Window::with_absolute_element_offset<R>(&mut self, offset: Point<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - Window::with_element_opacity<R>(&mut self, opacity: f32, f: impl FnOnce(&mut Self) -> R) -> R
    - Window::element_offset(&self) -> Point<Pixels>
    - Window::element_opacity(&self) -> f32

## Windows and Rendering

- Window: Manages the element tree, input, focus, and rendering.
    - Window::draw(&mut self, cx: &mut App)
    - Window::present(&self)
    - Window::defer_draw(&mut self, current_view: EntityId, priority: usize, element: impl IntoElement)
    - Window::paint_layer<R>(&mut self, bounds: Bounds<Pixels>, f: impl FnOnce(&mut Self) -> R) -> R
    - Window::paint_shadows(&mut self, shadows: &[BoxShadow], bounds: Bounds<Pixels>)
    - Window::paint_quad(&mut self, quad: PaintQuad)
    - Window::paint_path(&mut self, mut path: Path<Pixels>, color: impl Into<Background>)
    - Window::paint_underline(&mut self, underline: &Underline, bounds: Bounds<Pixels>)
    - paint_strikethrough(&mut self, strikethrough: &StrikethroughStyle, bounds: Bounds<Pixels>)
    - paint_glyph(&mut self, origin: Point<Pixels>, glyph_id: GlyphId, font_id: FontId, font_size: Pixels, color: Hsla)
    - paint_emoji(&mut self, origin: Point<Pixels>, glyph_id: GlyphId, font_id: FontId, font_size: Pixels)
    - paint_svg(&mut self, params: RenderSvgParams)
    - paint_image(&mut self, params: RenderImageParams)
    - paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVPixelBuffer)
    - drop_image(&mut self, data: Arc<RenderImage>) -> Result<()>
    - request_layout(&mut self, style: Style, children: impl IntoIterator<Item = LayoutId>, cx: &mut App) -> LayoutId
    - request_measured_layout<T>(&mut self, style: Style, measure_func: taffy::MeasureFunc, cx: &mut App) -> LayoutId
    - compute_layout(&mut self, layout_id: LayoutId, available_space: Size<AvailableSpace>, cx: &mut App) -> Size<Pixels>
    - layout_bounds(&mut self, layout_id: LayoutId) -> Bounds<Pixels>
    - insert_hitbox(&mut self, bounds: Bounds<Pixels>, opaque: bool) -> Hitbox
    - set_key_context(&mut self, context: KeyContext)
    - set_focus_handle(&mut self, focus_handle: &FocusHandle, _: &App)
    - set_view_id(&mut self, view_id: EntityId)
    - current_view(&self) -> EntityId
    - with_rendered_view<R>(&mut self, view_id: EntityId, f: impl FnOnce(&mut Self) -> R) -> R
    - with_image_cache<F, R>(&mut self, image_cache: Option<AnyImageCache>, f: F) -> R
    - handle_input(&mut self, handler: impl InputHandler + 'static)
    - on_mouse_event<Event: MouseEvent>(&mut self, listener: impl FnMut(&Event, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - on_key_event<Event: KeyEvent>(&mut self, listener: impl FnMut(&Event, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - on_modifiers_changed(&mut self, listener: impl FnMut(&ModifiersChangedEvent, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - on_focus_in(&mut self, listener: impl FnMut(&WindowFocusEvent, &mut Window, &mut App) + 'static) -> Subscription
    - on_focus_out(&mut self, listener: impl FnMut(&FocusOutEvent, &mut Window, &mut App) + 'static) -> Subscription
    - dispatch_keystroke(&mut self, keystroke: Keystroke, cx: &mut App) -> bool
    - keystroke_text_for(&self, action: &dyn Action) -> String
    - dispatch_event(&mut self, event: PlatformInput, cx: &mut App) -> DispatchEventResult
    - pending_input_keystrokes(&self) -> Option<&[Keystroke]>
    - observe_global<G: Global>(&mut self, mut on_notify: impl FnMut(&mut Window, &mut App) + 'static) -> Subscription
    - invalidate_character_coordinates(&self)
    - prompt(&mut self, level: PromptLevel, options: impl Into<PromptOptions>) -> RenderablePromptHandle
    - context_stack(&self) -> Vec<KeyContext>
    - available_actions(&self, cx: &App) -> Vec<Box<dyn Action>>
    - bindings_for_action(&self, action: &dyn Action) -> Vec<KeyBinding>
    - bindings_for_action_in(&self, action: &dyn Action, context: &[KeyContext]) -> Vec<KeyBinding>
    - bindings_for_action_in_context(&self, action: &dyn Action, context: &KeyContext) -> Vec<KeyBinding>
    - on_window_should_close(&self, mut on_should_close: impl FnMut(&mut App) -> bool + 'static) -> Subscription
    - on_action(&mut self, listener: impl Fn(&dyn Action, DispatchPhase, &mut Window, &mut App) + 'static) -> Subscription
    - toggle_inspector(&mut self, cx: &mut App)
    - is_inspector_picking(&self, _cx: &App) -> bool
    - with_inspector_state<T: 'static, R>(&mut self, f: impl FnOnce(Option<&mut T>, &mut Window, &mut App) -> (R, Option<T>)) -> R

## Concurrency

- [Executors](#executors)
- [Tasks](#tasks)

## Styled Trait Functions

Functions available on elements implementing the `Styled` trait for applying styles.

- `style(&mut self) -> &mut StyleRefinement`
    Returns a reference to the style memory of this element.

- **General Style Helpers** (generated by `gpui_macros::style_helpers!()`)
    - `w(width: impl Into<AbsoluteLength>) -> Self`
        Sets the width of the element.
    - `h(height: impl Into<AbsoluteLength>) -> Self`
        Sets the height of the element.
    - `size(size: impl Into<AbsoluteLength>) -> Self`
        Sets the width and height of the element to the same value.
    - `w_full() -> Self`
        Sets the width of the element to 100%.
    - `h_full() -> Self`
        Sets the height of the element to 100%.
    - `size_full() -> Self`
        Sets the width and height of the element to 100%.
    - `w_px() -> Self`
        Sets the width of the element to 1 pixel.
    - `h_px() -> Self`
        Sets the height of the element to 1 pixel.
    - `size_px() -> Self`
        Sets the width and height of the element to 1 pixel.

- **Visibility Style Methods** (generated by `gpui_macros::visibility_style_methods!()`)
    - `visible() -> Self`
        Sets the visibility of the element to `visible`.
            - `invisible() -> Self`
        Sets the visibility of the element to `hidden`.
        
- **Margin Style Methods** (generated by `gpui_macros::margin_style_methods!()`)
    Methods are generated for each side (top, bottom, left, right), horizontal and vertical axes, and all sides, with various predefined sizes (0, px, 0.5, 1, 1.5, ..., 96, auto, full).
    Examples:
    - `m(margin: impl Into<AbsoluteLength>) -> Self`
        Sets the margin on all sides of the element.
    - `mx(margin: impl Into<AbsoluteLength>) -> Self`
        Sets the horizontal margin of the element.
    - `mt_4() -> Self`
        Sets the top margin of the element to a predefined size (size 4).
    - `mb_auto() -> Self`
        Sets the bottom margin of the element to auto.

- **Padding Style Methods** (generated by `gpui_macros::padding_style_methods!()`)
    Methods are generated for each side (top, bottom, left, right), horizontal and vertical axes, and all sides, with various predefined sizes (0, px, 0.5, 1, 1.5, ..., 96, auto, full).
    Examples:
    - `p(padding: impl Into<AbsoluteLength>) -> Self`
        Sets the padding on all sides of the element.
    - `py(padding: impl Into<AbsoluteLength>) -> Self`
        Sets the vertical padding of the element.
    - `pl_8() -> Self`
        Sets the left padding of the element to a predefined size (size 8).
    - `pr_full() -> Self`
        Sets the right padding of the element to full.

- **Position Style Methods** (generated by `gpui_macros::position_style_methods!()`)
    - `relative() -> Self`
        Sets the position of the element to `relative`.
            - `absolute() -> Self`
        Sets the position of the element to `absolute`.
            Methods are also generated for setting inset, inset-x, inset-y, top, bottom, left, and right with various predefined sizes (0, px, 0.5, 1, 1.5, ..., 96, auto, full).
    Examples:
    - `inset(value: impl Into<AbsoluteLength>) -> Self`
        Sets the inset on all sides of the element.
    - `top_0() -> Self`
        Sets the top position of the element to 0.
    - `left_auto() -> Self`
        Sets the left position of the element to auto.

- **Overflow Style Methods** (generated by `gpui_macros::overflow_style_methods!()`)
    - `overflow_hidden() -> Self`
        Sets the behavior of content that overflows the container to be hidden.
            - `overflow_x_hidden() -> Self`
        Sets the behavior of content that overflows the container on the X axis to be hidden.
            - `overflow_y_hidden() -> Self`
        Sets the behavior of content that overflows the container on the Y axis to be hidden.
        
- **Cursor Style Methods** (generated by `gpui_macros::cursor_style_methods!()`)
    - `cursor(cursor: CursorStyle) -> Self`
        Set the cursor style when hovering over this element.
    - `cursor_default() -> Self`
        Sets the cursor style when hovering an element to `default`.
            - `cursor_pointer() -> Self`
        Sets the cursor style when hovering an element to `pointer`.
            - `cursor_text() -> Self`
        Sets cursor style when hovering over an element to `text`.
            - `cursor_move() -> Self`
        Sets cursor style when hovering over an element to `move`.
            - `cursor_not_allowed() -> Self`
        Sets cursor style when hovering over an element to `not-allowed`.
            - `cursor_context_menu() -> Self`
        Sets cursor style when hovering over an element to `context-menu`.
            - `cursor_crosshair() -> Self`
        Sets cursor style when hovering over an element to `crosshair`.
            - `cursor_vertical_text() -> Self`
        Sets cursor style when hovering over an element to `vertical-text`.
            - `cursor_alias() -> Self`
        Sets cursor style when hovering over an element to `alias`.
            - `cursor_copy() -> Self`
        Sets cursor style when hovering over an element to `copy`.
            - `cursor_no_drop() -> Self`
        Sets cursor style when hovering over an element to `no-drop`.
            - `cursor_grab() -> Self`
        Sets cursor style when hovering over an element to `grab`.
            - `cursor_grabbing() -> Self`
        Sets cursor style when hovering over an element to `grabbing`.
            - `cursor_ew_resize() -> Self`
        Sets cursor style when hovering over an element to `ew-resize`.
            - `cursor_ns_resize() -> Self`
        Sets cursor style when hovering over an element to `ns-resize`.
            - `cursor_nesw_resize() -> Self`
        Sets cursor style when hovering over an element to `nesw-resize`.
            - `cursor_nwse_resize() -> Self`
        Sets cursor style when hovering over an element to `nwse-resize`.
            - `cursor_col_resize() -> Self`
        Sets cursor style when hovering over an element to `col-resize`.
            - `cursor_row_resize() -> Self`
        Sets cursor style when hovering over an element to `row-resize`.
            - `cursor_n_resize() -> Self`
        Sets cursor style when hovering over an element to `n-resize`.
            - `cursor_e_resize() -> Self`
        Sets cursor style when hovering over an element to `e-resize`.
            - `cursor_s_resize() -> Self`
        Sets cursor style when hovering over an element to `s-resize`.
            - `cursor_w_resize() -> Self`
        Sets cursor style when hovering over an element to `w-resize`.
            - `cursor_none(cursor: CursorStyle) -> Self`
        Sets cursor style when hovering over an element to `none`.
        
- **Border Style Methods** (generated by `gpui_macros::border_style_methods!()`)
    Methods are generated for setting border width on each side (top, bottom, left, right), horizontal and vertical axes, and all sides, with predefined sizes (0, 2, 4, 8).
    Examples:
    - `border_0() -> Self`
        Sets the border width on all sides to 0.
    - `border_t_2() -> Self`
        Sets the top border width to 2.
    - `border_color(border_color: impl Into<Hsla>) -> Self`
        Sets the border color of the element.
    Methods are also generated for setting border style on each side (top, bottom, left, right), horizontal and vertical axes, and all sides, with predefined styles (solid, dashed, dotted, double, hidden, none).
    Examples:
    - `border_solid() -> Self`
        Sets the border style on all sides to solid.
    - `border_b_dashed() -> Self`
        Sets the bottom border style to dashed.

- **Box Shadow Style Methods** (generated by `gpui_macros::box_shadow_style_methods!()`)
    - `shadow(shadows: std::vec::Vec<gpui::BoxShadow>) -> Self`
        Sets the box shadow of the element.
            - `shadow_none() -> Self`
        Clears the box shadow of the element.
            - `shadow_sm() -> Self`
        Sets a small box shadow on the element.
            - `shadow_md() -> Self`
        Sets a medium box shadow on the element.
            - `shadow_lg() -> Self`
        Sets a large box shadow on the element.
            - `shadow_xl() -> Self`
        Sets an extra large box shadow on the element.
            - `shadow_2xl() -> Self`
        Sets a very large box shadow on the element.
        
- `block() -> Self`
    Sets the display type of the element to `block`.
    
- `flex() -> Self`
    Sets the display type of the element to `flex`.
    
- `whitespace_normal() -> Self`
    Sets the whitespace of the element to `normal`.
    
- `whitespace_nowrap() -> Self`
    Sets the whitespace of the element to `nowrap`.
    
- `text_ellipsis() -> Self`
    Sets the truncate overflowing text with an ellipsis (…) if needed.
    
- `text_overflow(overflow: TextOverflow) -> Self`
    Sets the text overflow behavior of the element.

- `text_align(align: TextAlign) -> Self`
    Set the text alignment of the element.

- `text_left() -> Self`
    Sets the text alignment to left

- `text_center() -> Self`
    Sets the text alignment to center

- `text_right() -> Self`
    Sets the text alignment to right

- `truncate() -> Self`
    Sets the truncate to prevent text from wrapping and truncate overflowing text with an ellipsis (…) if needed.
    
- `line_clamp(lines: usize) -> Self`
    Sets number of lines to show before truncating the text.
    
- `flex_col() -> Self`
    Sets the flex direction of the element to `column`.
    
- `flex_col_reverse() -> Self`
    Sets the flex direction of the element to `column-reverse`.
    
- `flex_row() -> Self`
    Sets the flex direction of the element to `row`.
    
- `flex_row_reverse() -> Self`
    Sets the flex direction of the element to `row-reverse`.
    
- `flex_1() -> Self`
    Sets the element to allow a flex item to grow and shrink as needed, ignoring its initial size.
    
- `flex_auto() -> Self`
    Sets the element to allow a flex item to grow and shrink, taking into account its initial size.
    
- `flex_initial() -> Self`
    Sets the element to allow a flex item to shrink but not grow, taking into account its initial size.
    
- `flex_none() -> Self`
    Sets the element to prevent a flex item from growing or shrinking.
    
- `flex_basis(basis: impl Into<Length>) -> Self`
    Sets the initial size of flex items for this element.
    
- `flex_grow() -> Self`
    Sets the element to allow a flex item to grow to fill any available space.
    
- `flex_shrink() -> Self`
    Sets the element to allow a flex item to shrink if needed.
    
- `flex_shrink_0() -> Self`
    Sets the element to prevent a flex item from shrinking.
    
- `flex_wrap() -> Self`
    Sets the element to allow flex items to wrap.
    
- `flex_wrap_reverse() -> Self`
    Sets the element wrap flex items in the reverse direction.
    
- `flex_nowrap() -> Self`
    Sets the element to prevent flex items from wrapping, causing inflexible items to overflow the container if necessary.
    
- `items_start() -> Self`
    Sets the element to align flex items to the start of the container's cross axis.
    
- `items_end() -> Self`
    Sets the element to align flex items to the end of the container's cross axis.
    
- `items_center() -> Self`
    Sets the element to align flex items along the center of the container's cross axis.
    
- `items_baseline() -> Self`
    Sets the element to align flex items along the baseline of the container's cross axis.
    
- `justify_start() -> Self`
    Sets the element to justify flex items against the start of the container's main axis.
    
- `justify_end() -> Self`
    Sets the element to justify flex items against the end of the container's main axis.
    
- `justify_center() -> Self`
    Sets the element to justify flex items along the center of the container's main axis.
    
- `justify_between() -> Self`
    Sets the element to justify flex items along the container's main axis
    such that there is an equal amount of space between each item.
    
- `justify_around() -> Self`
    Sets the element to justify items along the container's main axis such
    that there is an equal amount of space on each side of each item.
    
- `content_normal() -> Self`
    Sets the element to pack content items in their default position as if no align-content value was set.
    
- `content_center() -> Self`
    Sets the element to pack content items in the center of the container's cross axis.
    
- `content_start() -> Self`
    Sets the element to pack content items against the start of the container's cross axis.
    
- `content_end() -> Self`
    Sets the element to pack content items against the end of the container's cross axis.
    
- `content_between() -> Self`
    Sets the element to pack content items along the container's cross axis
    such that there is an equal amount of space between each item.
    
- `content_around() -> Self`
    Sets the element to pack content items along the container's cross axis
    such that there is an equal amount of space on each side of each item.
    
- `content_evenly() -> Self`
    Sets the element to pack content items along the container's cross axis
    such that there is an equal amount of space between each item.
    
- `content_stretch() -> Self`
    Sets the element to allow content items to fill the available space along the container's cross axis.
    
- `bg<F>(fill: F) -> Self where F: Into<Fill>, Self: Sized,`
    Sets the background color of the element.

- `border_dashed() -> Self`
    Sets the border style of the element.

- `text_style(&mut self) -> &mut Option<TextStyleRefinement>`
    Returns a mutable reference to the text style that has been configured on this element.

- `text_color(color: impl Into<Hsla>) -> Self`
    Sets the text color of this element. This value cascades to its child elements.

- `font_weight(weight: FontWeight) -> Self`
    Sets the font weight of this element. This value cascades to its child elements.

- `text_bg(bg: impl Into<Hsla>) -> Self`
    Sets the background color of this element. This value cascades to its child elements.

- `text_size(size: impl Into<AbsoluteLength>) -> Self`
    Sets the text size of this element. This value cascades to its child elements.

- `text_xs() -> Self`
    Sets the text size to 'extra small'.
    
- `text_sm() -> Self`
    Sets the text size to 'small'.
    
- `text_base() -> Self`
    Sets the text size to 'base'.
    
- `text_lg() -> Self`
    Sets the text size to 'large'.
    
- `text_xl() -> Self`
    Sets the text size to 'extra large'.
    
- `text_2xl() -> Self`
    Sets the text size to 'extra extra large'.
    
- `text_3xl() -> Self`
    Sets the text size to 'extra extra extra large'.
    
- `italic() -> Self`
    Sets the font style of the element to italic.
    
- `not_italic() -> Self`
    Sets the font style of the element to normal (not italic).
    
- `underline() -> Self`
    Sets the text decoration to underline.
    
- `line_through() -> Self`
    Sets the decoration of the text to have a line through it.
    
- `text_decoration_none() -> Self`
    Removes the text decoration on this element. This value cascades to its child elements.

- `text_decoration_color(color: impl Into<Hsla>) -> Self`
    Sets the color for the underline on this element.

- `text_decoration_solid() -> Self`
    Sets the text decoration style to a solid line.
    
- `text_decoration_wavy() -> Self`
    Sets the text decoration style to a wavy line.
    
- `text_decoration_0() -> Self`
    Sets the text decoration to be 0px thick.
    
- `text_decoration_1() -> Self`
    Sets the text decoration to be 1px thick.
    
- `text_decoration_2() -> Self`
    Sets the text decoration to be 2px thick.
    
- `text_decoration_4() -> Self`
    Sets the text decoration to be 4px thick.
    
- `text_decoration_8() -> Self`
    Sets the text decoration to be 8px thick.
    
- `font_family(family_name: impl Into<SharedString>) -> Self`
    Sets the font family of this element and its children.

- `font(font: Font) -> Self`
    Sets the font of this element and its children.

- `line_height(line_height: impl Into<DefiniteLength>) -> Self`
    Sets the line height of this element and its children.

- `opacity(opacity: f32) -> Self`
    Sets the opacity of this element and its children.

- `debug() -> Self`
    Draws a debug border around this element.

- `debug_below() -> Self`
    Draws a debug border on all conforming elements below this element.